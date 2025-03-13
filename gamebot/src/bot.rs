use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc::{channel, Sender, Receiver}, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

pub struct Bot {
    pub(crate) connection: TcpStream,
    pub(crate) name: String,
    stop_signal: Arc<Mutex<Option<Sender<()>>>>,
}

impl Bot {
    pub fn new(connection: TcpStream, name: String) -> Self {
        Bot {
            connection,
            name,
            stop_signal: Arc::new(Mutex::new(None)),
        }
    }

    pub fn connect(&mut self) {
        TcpStream::connect("172.17.0.1:6969").unwrap();
        println!("NAME={}", self.name);
        self.connection.write_all(format!("NAME={}",self.name).as_bytes()).unwrap();
    }

    // Optimisé pour envoyer des commandes plus rapidement
    pub fn send_command(&mut self, command: &str) {
        let cmd_with_newline = if command.ends_with('\n') {
            command.to_string()
        } else {
            format!("{}\n", command)
        };

        if let Err(e) = self.connection.write_all(cmd_with_newline.as_bytes()) {
            eprintln!("Erreur d'envoi de commande: {:?}", e);
        } else {
            let _ = self.connection.flush();
        }
    }

    // Optimisé pour envoyer plusieurs commandes d'un coup
    fn send_commands(&mut self, commands: &[&str]) {
        let combined_command = commands.join("#");
        self.send_command(&combined_command);
    }

    fn mot_l(&mut self, pow: f32) {
        self.send_command(&format!("MotL={}", pow));
    }

    fn mot_r(&mut self, pow: f32) {
        self.send_command(&format!("MotR={}", pow));
    }

    // Optimisé pour envoyer les commandes moteur en une seule fois
    fn set_motors(&mut self, left: f32, right: f32) {
        self.send_commands(&[
            &format!("MotL={}", left),
            &format!("MotR={}", right)
        ]);
    }

    pub fn shoot(&mut self, shoot: bool) {
        let trigger_value = if shoot { "1.0" } else { "0.0" };
        self.send_command(&format!("GunTrig={}", trigger_value));
    }

    pub fn aim_gun(&mut self, traverse: f32) {
        let traverse_value = traverse.max(0.0).min(1.0);
        self.send_command(&format!("GunTrav={}", traverse_value));
    }

    // Optimisé pour régler le tir et la visée en une seule commande
    pub fn aim_and_shoot(&mut self, traverse: f32, shoot: bool) {
        let traverse_value = traverse.max(0.0).min(1.0);
        let trigger_value = if shoot { "1.0" } else { "0.0" };

        self.send_commands(&[
            &format!("GunTrav={}", traverse_value),
            &format!("GunTrig={}", trigger_value)
        ]);
    }

    pub fn get_player_list(&mut self) -> Vec<String> {
        self.send_command("NLIST");

        let mut buffer = [0; 512];
        let _ = self.connection.try_clone().unwrap().set_read_timeout(Some(Duration::from_millis(20)));

        match self.connection.try_clone().unwrap().read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                response.split('=').map(|s| s.to_string()).collect()
            },
            _ => vec![],
        }
    }

    pub fn is_enemy(&mut self, name: &str) -> bool {
        // Vérifier si le nom commence par "TeamAlpha-"
        !name.starts_with("TeamAlpha-")
    }

    // Méthode principale pour suivre et tirer sur les ennemis - optimisée pour la vitesse
    pub fn track_and_shoot_enemy(&mut self) {
        self.connection.write_all(format!("NAME={}",self.name).as_bytes()).unwrap();
        let mut bot = self.clone();
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();

        {
            let mut stop_signal = self.stop_signal.lock().unwrap();
            *stop_signal = Some(tx);
        }

        thread::spawn(move || {
            let mut is_firing = false;
            let mut last_enemy_data: Option<(f32, f32)> = None;
            let mut no_enemy_counter = 0;
            let mut cproj_check_counter = 0;
            let mut needs_evasion = false;

            // Message initial pour le jeu
            bot.send_command("MSG=Chasse en cours!");

            loop {
                if rx.try_recv().is_ok() {
                    println!("Arrêt de la poursuite!");
                    bot.set_motors(0.5, 0.5);  // Arrêt complet
                    bot.shoot(false);
                    break;
                }

                // Vérifier les projectiles à éviter tous les 5 cycles
                cproj_check_counter += 1;
                if cproj_check_counter >= 5 {
                    cproj_check_counter = 0;
                    needs_evasion = bot.check_for_incoming_projectiles();
                }

                // Si un projectile arrive, priorité à l'évitement
                if needs_evasion {
                    bot.evasive_maneuvers();
                    is_firing = false;  // Ne pas tirer pendant l'évitement
                    bot.shoot(false);

                    // Continuer la boucle sans chercher d'ennemis pendant l'évitement
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }

                // Obtient les informations sur l'ennemi le plus proche - optimisé
                if let Some((orientation, distance)) = bot.get_closest_enemy_fast() {
                    no_enemy_counter = 0;
                    last_enemy_data = Some((orientation, distance));

                    // Poursuite à pleine vitesse
                    bot.hunt_enemy_max_speed(orientation, distance);

                    // Calcul du tir optimisé
                    let gun_traverse = bot.calculate_optimal_gun_position(orientation, distance);
                    let should_fire = bot.should_fire_precise(orientation, distance);

                    // Combine les commandes de tir et d'orientation du canon
                    bot.aim_and_shoot(gun_traverse, should_fire);

                    // Met à jour l'état du tir
                    is_firing = should_fire;

                    // Affichage du statut
                    if should_fire && distance < 100.0 {
                        bot.send_command("MSG=Cible acquise - FEU!");
                    } else {
                        bot.send_command(&format!("MSG=Poursuite: {}m", distance as i32));
                    }
                } else {
                    no_enemy_counter += 1;

                    // Continuer la poursuite dans la dernière direction connue pendant un court moment
                    if no_enemy_counter < 5 && last_enemy_data.is_some() {
                        let (last_orientation, last_distance) = last_enemy_data.unwrap();
                        bot.hunt_enemy_max_speed(last_orientation, last_distance);
                        // Maintenir le tir si on tirait déjà et que l'ennemi était proche
                        if is_firing && last_distance < 100.0 {
                            bot.shoot(true);
                        } else {
                            bot.shoot(false);
                            is_firing = false;
                        }
                        bot.send_command("MSG=Reprise de contact...");
                    } else {
                        // Recherche rapide et agressive
                        bot.search_max_speed();
                        if is_firing {
                            bot.shoot(false);
                            is_firing = false;
                        }
                        bot.send_command("MSG=Recherche de cibles");
                        last_enemy_data = None;
                    }
                }

                // Pause minimale pour maximiser la fréquence d'exécution
                thread::sleep(Duration::from_millis(5));
            }
        });
    }


    // Vérifie s'il y a des projectiles à proximité qui nécessitent une évitement
    fn check_for_incoming_projectiles(&mut self) -> bool {
        self.send_command("CPROJ");

        let mut buffer = [0; 512];
        if let Ok(bytes_read) = self.connection.try_clone().unwrap().read(&mut buffer) {
            if bytes_read > 0 {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                if !response.contains("EMPTY") {
                    for line in response.lines() {
                        if line.contains("/") {
                            let parts: Vec<&str> = line.trim().split('/').collect();
                            if parts.len() == 2 {
                                if let (Ok(orientation), Ok(distance)) =
                                    (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                                    // Si un projectile est proche et se dirige vers nous
                                    if distance < 70.0 {
                                        println!("Projectile détecté! Distance: {}, Orientation: {}", distance, orientation);
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    // Manœuvres d'évitement de projectiles
    fn evasive_maneuvers(&mut self) {
        static mut EVASION_PATTERN: u8 = 0;
        static mut EVASION_COUNTER: u8 = 0;

        unsafe {
            EVASION_COUNTER += 1;

            // Changer de motif d'évitement après plusieurs itérations
            if EVASION_COUNTER > 20 {
                EVASION_PATTERN = (EVASION_PATTERN + 1) % 4;
                EVASION_COUNTER = 0;
            }

            // Différents motifs d'évitement pour être imprévisible
            match EVASION_PATTERN {
                0 => {
                    // Virage serré à gauche
                    self.set_motors(0.0, 1.0);
                },
                1 => {
                    // Virage serré à droite
                    self.set_motors(1.0, 0.0);
                },
                2 => {
                    // Reculer rapidement en zigzag
                    if EVASION_COUNTER % 2 == 0 {
                        self.set_motors(0.3, 0.1);
                    } else {
                        self.set_motors(0.1, 0.3);
                    }
                },
                _ => {
                    // Virage et accélération rapides
                    self.set_motors(0.0, 0.8);
                }
            }
        }
    }

    pub fn get_closest_enemy_fast(&mut self) -> Option<(f32, f32)> {
        let player_list = self.get_player_list();
        let mut closest_enemy: Option<(f32, f32)> = None;

        for player_name in player_list {
            if !player_name.starts_with("TeamAlpha-") {
                self.send_command(&format!("NBOT={}", player_name));

                let mut buffer = [0; 512];
                let _ = self.connection.try_clone().unwrap().set_read_timeout(Some(Duration::from_millis(20)));

                match self.connection.try_clone().unwrap().read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                        if !response.contains("EMPTY") {
                            for line in response.lines() {
                                if line.contains("/") {
                                    let parts: Vec<&str> = line.trim().split('/').collect();
                                    if parts.len() == 2 {
                                        if let (Ok(orientation), Ok(distance)) =
                                            (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                                            // Garder l'ennemi le plus proche
                                            if closest_enemy.is_none() || distance < closest_enemy.unwrap().1 {
                                                closest_enemy = Some((orientation, distance));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        closest_enemy
    }
    // Poursuite à vitesse maximale
    pub fn hunt_enemy_max_speed(&mut self, orientation: f32, distance: f32) {
        // Convertir l'orientation en angle relatif où 0.5 = devant
        let rel_orientation = if orientation > 0.5 {
            orientation - 1.0
        } else {
            orientation
        };

        // Si l'ennemi est presque devant, avancer tout droit
        if rel_orientation.abs() < 0.1 {
            self.set_motors(1.0, 1.0);
        } else {
            // Sinon, ajuster les moteurs pour tourner vers l'ennemi
            let turn_intensity = rel_orientation.abs() * 2.0; // Facteur de virage
            let min_power = 0.7; // Puissance minimale pour maintenir la vitesse

            if rel_orientation < 0.0 {
                // Ennemi à gauche - virage à gauche
                let left_power = (1.0 - turn_intensity).max(min_power);
                self.set_motors(left_power, 1.0);
            } else {
                // Ennemi à droite - virage à droite
                let right_power = (1.0 - turn_intensity).max(min_power);
                self.set_motors(1.0, right_power);
            }
        }
    }


    // Recherche à vitesse maximale
    pub fn search_max_speed(&mut self) {
        static mut SEARCH_TARGET_ORIENTATION: f32 = 0.0;
        static mut SEARCH_STATE: u8 = 0; // 0: avancer, 1: tourner
        static mut LAST_ORIENTATION: f32 = 0.0;

        unsafe {
            // Obtenir l'orientation actuelle du bot
            self.send_command("ORIENT");
            let mut buffer = [0; 512];
            if let Ok(bytes_read) = self.connection.try_clone().unwrap().read(&mut buffer) {
                if bytes_read > 0 {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                    if let Ok(current_orientation) = response.trim().parse::<f32>() {
                        match SEARCH_STATE {
                            0 => {
                                // Avancer tout droit pendant un certain temps
                                self.set_motors(1.0, 1.0);

                                // Après un délai, passer à l'état de virage
                                if current_orientation - LAST_ORIENTATION < 0.1 {
                                    SEARCH_STATE = 1;
                                    SEARCH_TARGET_ORIENTATION = current_orientation + 1.57; // Tourner de 90°
                                }
                            },
                            1 => {
                                // Tourner jusqu'à atteindre la cible d'orientation
                                if (current_orientation - SEARCH_TARGET_ORIENTATION).abs() > 0.1 {
                                    if current_orientation < SEARCH_TARGET_ORIENTATION {
                                        self.set_motors(0.5, 1.0); // Tourner à droite
                                    } else {
                                        self.set_motors(1.0, 0.5); // Tourner à gauche
                                    }
                                } else {
                                    SEARCH_STATE = 0; // Retour à l'état d'avancement
                                }
                            },
                            _ => {
                                SEARCH_STATE = 0;
                            }
                        }
                        LAST_ORIENTATION = current_orientation;
                    }
                }
            }
        }
    }

    // Calcul optimisé et précis pour la position du canon
    pub fn calculate_optimal_gun_position(&self, orientation: f32, distance: f32) -> f32 {
        // Normaliser l'orientation dans [-0.5, 0.5]
        let rel_orientation = if orientation > 0.5 {
            orientation - 1.0
        } else {
            orientation
        } - 0.5;

        // Plus grande compensation pour les ennemis lointains ou en mouvement rapide
        let distance_factor = (distance / 100.0).min(2.0);

        // Légère anticipation du mouvement - augmente avec la distance
        let movement_prediction = 0.02 * distance_factor * rel_orientation.signum();

        // Position de base du canon (0.5 = centre)
        let base_position = 0.5 + rel_orientation;

        // Position finale avec compensation du mouvement
        let final_position = base_position + movement_prediction;

        // Limites de sécurité
        final_position.max(0.0).min(1.0)
    }

    // Logique de tir améliorée et beaucoup plus précise
    pub fn should_fire_precise(&self, orientation: f32, distance: f32) -> bool {
        // Constantes optimisées
        const MAX_FIRE_DISTANCE: f32 = 500.0; // Distance max de tir augmentée

        // Vérifier d'abord si la cible est à portée
        if distance > MAX_FIRE_DISTANCE {
            return false;
        }

        // Normaliser l'orientation pour faciliter les calculs
        let rel_orientation = if orientation > 0.5 {
            orientation - 1.0
        } else {
            orientation
        } - 0.5; // Dans la plage [-0.5, 0.5]

        // La précision requise dépend de la distance
        // Plus la cible est loin, plus il faut être précis
        let precision_requirement = match distance as u32 {
            0..=50 => 0.15,    // Très proche: tolérance large
            51..=100 => 0.10,  // Proche: tolérance moyenne
            101..=200 => 0.05, // Moyenne distance: précision importante
            _ => 0.025         // Longue distance: haute précision
        };

        // Tir seulement si l'orientation est dans la tolérance
        rel_orientation.abs() < precision_requirement
    }

    // Méthode d'arrêt du tracking
    pub fn stop_tracking(&self) {
        if let Some(ref tx) = *self.stop_signal.lock().unwrap() {
            let _ = tx.send(());
        }
    }
}

// Implémentation de Clone pour Bot
impl Clone for Bot {
    fn clone(&self) -> Self {
        Bot {
            connection: self.connection.try_clone().expect("Failed to clone TcpStream"),
            name: self.name.clone(),
            stop_signal: Arc::clone(&self.stop_signal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_bot() {
        let connection = TcpStream::connect("172.17.0.1:6969").unwrap();
        let mut bot = Bot::new(connection, "fabrice".to_string());

        assert_eq!(bot.name, "fabrice");
    }
}