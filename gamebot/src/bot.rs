use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{mpsc::{channel, Sender, Receiver}, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

pub(crate) struct Bot {
    pub(crate) connection: TcpStream,
    pub(crate) name: String,
    stop_signal: Arc<Mutex<Option<Sender<()>>>>,
}

impl Bot {
    pub fn new(connection: TcpStream, name: String) -> Self {
        Bot {
            connection,
            name,
            stop_signal: Arc::new(Mutex::new(None)), // Initialisation
        }
    }

    pub fn connect(&self) {
        TcpStream::connect("172.17.0.1:6969").unwrap();
    }

    fn mot_l(&mut self, pow: f32) {
        let _ = self.connection.write_all(format!("MotL={}\n", pow).as_bytes());
    }

    fn mot_r(&mut self, pow: f32) {
        let _ = self.connection.write_all(format!("MotR={}\n", pow).as_bytes());
    }

    pub fn start_moving(&mut self) {
        let mut bot = self.clone();
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();

        // Stocke le sender pour arrêter le thread plus tard
        {
            let mut stop_signal = self.stop_signal.lock().unwrap();
            *stop_signal = Some(tx);
        }

        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                // Vérifie si un signal de stop a été envoyé
                if rx.try_recv().is_ok() {
                    println!("Stop signal reçu, arrêt du thread!");
                    break;
                }

                // Génération de puissances aléatoires
                let pow = rng.gen_range(0.0..=1.0);
                bot.mot_l(pow);
                bot.mot_r(pow);

                // Pause pour simuler un mouvement continu
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop_moving(&self) {
        // Envoie le signal d'arrêt via le Sender
        if let Some(ref tx) = *self.stop_signal.lock().unwrap() {
            let _ = tx.send(());
        }
    }

        // Méthode pour trouver et cibler l'ennemi le plus proche
        // Méthode optimisée pour trouver et cibler l'ennemi le plus proche
        pub fn track_closest_enemy(&mut self) {
            let mut bot = self.clone();
            let (tx, rx): (Sender<()>, Receiver<()>) = channel();

            // Stocke le sender pour arrêter le thread plus tard
            {
                let mut stop_signal = self.stop_signal.lock().unwrap();
                *stop_signal = Some(tx);
            }

            thread::spawn(move || {
                // Arrêter les moteurs d'abord pour repartir de zéro
                bot.mot_l(0.5);
                bot.mot_r(0.5);

                loop {
                    // Vérifie si un signal de stop a été envoyé (non bloquant)
                    if rx.try_recv().is_ok() {
                        println!("Stop signal reçu, arrêt du suivi d'ennemi!");
                        bot.mot_l(0.5); // Arrêt des moteurs
                        bot.mot_r(0.5);
                        break;
                    }

                    // Obtient les informations sur l'ennemi le plus proche - SYNCHRONE
                    if let Some((orientation, distance)) = bot.get_closest_enemy_fast() {
                        // Ajuste les moteurs pour se diriger vers l'ennemi - RÉACTION IMMÉDIATE
                        bot.move_towards_enhanced(orientation, distance);

                        println!("Ennemi détecté: orientation={}, distance={}", orientation, distance);
                    } else {
                        // Si aucun ennemi n'est trouvé, on tourne sur place pour chercher
                        bot.rotate_search();
                        println!("Aucun ennemi détecté, recherche en cours...");
                    }

                    // Pause plus courte pour réagir plus rapidement
                    thread::sleep(Duration::from_millis(20));
                }
            });
        }

    // Méthode pour trouver, cibler et tirer sur l'ennemi le plus proche
    pub fn shoot(&mut self, shoot: bool) {
        let trigger_value = if shoot { "1.0" } else { "0.0" };
        let command = format!("GunTrig={}\n", trigger_value);

        if let Err(e) = self.connection.write_all(command.as_bytes()) {
            eprintln!("Erreur lors de l'envoi de la commande de tir: {:?}", e);
        } else {
            let _ = self.connection.flush();
            if shoot {
                println!("Canon activé!");
            } else {
                println!("Canon désactivé!");
            }
        }
    }

    // Fonction pour ajuster la traverse du canon
    pub fn aim_gun(&mut self, traverse: f32) {
        // Assurer que la valeur est entre 0 et 1
        let traverse_value = traverse.max(0.0).min(1.0);
        let command = format!("GunTrav={}\n", traverse_value);

        if let Err(e) = self.connection.write_all(command.as_bytes()) {
            eprintln!("Erreur lors de l'ajustement du canon: {:?}", e);
        } else {
            let _ = self.connection.flush();
        }
    }

    // Fonction pour trouver et tirer sur un ennemi avec déplacement amélioré
    pub fn track_and_shoot_enemy(&mut self) {
        let mut bot = self.clone();
        let (tx, rx): (Sender<()>, Receiver<()>) = channel();

        // Stocke le sender pour arrêter le thread plus tard
        {
            let mut stop_signal = self.stop_signal.lock().unwrap();
            *stop_signal = Some(tx);
        }

        thread::spawn(move || {
            // Variables pour la gestion du tir et du mouvement
            let mut last_direction_change = std::time::Instant::now();
            let mut direction = 1; // 1 pour avancer, -1 pour reculer
            let mut is_firing = false;

            loop {
                // Vérifie si un signal de stop a été envoyé
                if rx.try_recv().is_ok() {
                    println!("Stop signal reçu, arrêt du suivi d'ennemi!");
                    bot.mot_l(0.5); // Arrêt des moteurs
                    bot.mot_r(0.5);
                    bot.shoot(false); // Arrêt du tir
                    break;
                }

                // Requête CBOT pour l'ennemi le plus proche
                if let Err(e) = bot.connection.write_all("CBOT\n".as_bytes()) {
                    eprintln!("Erreur lors de la requête CBOT: {:?}", e);
                    continue;
                }

                bot.connection.flush().unwrap();

                let mut response = String::new();
                // Lire la réponse
                // Note: Vous devrez peut-être ajuster cette partie selon la façon dont
                // votre code gère la lecture des réponses du serveur
                let mut buffer = [0; 1024];
                match bot.connection.read(&mut buffer) {
                    Ok(n) => {
                        response = String::from_utf8_lossy(&buffer[0..n]).to_string();
                    },
                    Err(e) => {
                        eprintln!("Erreur lors de la lecture de la réponse: {:?}", e);
                        continue;
                    }
                }

                // Analyser la réponse pour obtenir orientation et distance
                if response == "EMPTY" || response.trim().is_empty() {
                    // Aucun ennemi trouvé, on fait une recherche agressive
                    bot.search_aggressively();
                    if is_firing {
                        bot.shoot(false);
                        is_firing = false;
                    }
                    println!("Aucun ennemi détecté, recherche en cours...");
                } else {
                    // Format attendu: orientation/distance
                    let parts: Vec<&str> = response.trim().split('/').collect();
                    if parts.len() >= 2 {
                        let orientation: f32 = match parts[0].parse() {
                            Ok(val) => val,
                            Err(_) => continue
                        };

                        let distance: f32 = match parts[1].parse() {
                            Ok(val) => val,
                            Err(_) => continue
                        };

                        // Changement de direction périodique pour être moins prévisible
                        if last_direction_change.elapsed() > Duration::from_secs(3) {
                            direction = if rand::random::<bool>() { 1 } else { -1 };
                            last_direction_change = std::time::Instant::now();
                        }

                        // Déplacement plus agressif
                        bot.move_aggressively(orientation, distance, direction);

                        // Ajuster la position du canon
                        bot.aim_gun(orientation);

                        // Détermine si on doit tirer
                        let should_fire = bot.should_fire(orientation, distance);

                        // Tir si nécessaire
                        if should_fire && !is_firing {
                            bot.shoot(true);
                            is_firing = true;
                            println!("Tir activé! orientation={:.3}, distance={:.1}", orientation, distance);
                        } else if !should_fire && is_firing {
                            bot.shoot(false);
                            is_firing = false;
                            println!("Tir désactivé!");
                        }

                        println!("Ennemi: orient={:.3}, dist={:.1}", orientation, distance);
                    }
                }

                // Pause courte pour réagir rapidement
                thread::sleep(Duration::from_millis(20));
            }
        });
    }

    // Déplacement agressif vers l'ennemi
    fn move_aggressively(&mut self, orientation: f32, distance: f32, direction: i8) {
        // L'orientation est déjà normalisée entre 0 et 1 selon la doc

        // Facteur d'aggressivité - plus il est élevé, plus le mouvement est rapide
        let aggression_factor = if distance < 100.0 { 1.0 } else { 1.0 };

        // Coefficient de direction (avancer ou reculer)
        let dir_coeff = direction as f32;

        // Conversion de l'orientation (0.5 = devant, 0.25 = gauche, 0.75 = droite)
        // à une orientation relative (-0.25 = gauche, 0.25 = droite)
        let rel_orientation = if orientation > 0.5 {
            orientation - 0.5
        } else {
            orientation + 0.5
        } - 0.5;

        // Si l'ennemi est à gauche
        if rel_orientation < -0.1 {
            // Tourner rapidement à gauche
            let left_power = 0.5 - (aggression_factor * dir_coeff * 0.5);
            let right_power = 0.5 + (aggression_factor * dir_coeff * 0.5);
            self.mot_l(left_power.max(0.0).min(1.0));
            self.mot_r(right_power.max(0.0).min(1.0));
        }
        // Si l'ennemi est à droite
        else if rel_orientation > 0.1 {
            // Tourner rapidement à droite
            let left_power = 0.5 + (aggression_factor * dir_coeff * 0.5);
            let right_power = 0.5 - (aggression_factor * dir_coeff * 0.5);
            self.mot_l(left_power.max(0.0).min(1.0));
            self.mot_r(right_power.max(0.0).min(1.0));
        }
        // Si l'ennemi est plus ou moins devant
        else {
            // Avancer/reculer à pleine vitesse
            let base_speed = 0.5 + (0.4 * dir_coeff);
            self.mot_l(base_speed);
            self.mot_r(base_speed);
        }
    }

    // Recherche agressive en l'absence d'ennemis
    fn search_aggressively(&mut self) {
        // Rotation rapide en alternant la direction
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if timestamp % 4 < 2 {
            // Tourner à gauche rapidement
            self.mot_l(0.2);
            self.mot_r(0.8);
        } else {
            // Tourner à droite rapidement
            self.mot_l(0.8);
            self.mot_r(0.2);
        }
    }

    // Méthode pour déterminer s'il faut tirer
    fn should_fire(&self, orientation: f32, distance: f32) -> bool {
        // Distance maximale de tir efficace
        const MAX_FIRING_DISTANCE: f32 = 30.0;

        // Tolérance d'angle pour le tir
        const ANGLE_TOLERANCE: f32 = 0.15;

        // Vérifier la distance
        let is_in_range = distance < MAX_FIRING_DISTANCE;

        // Calculer l'orientation relative pour viser
        let rel_orientation = if orientation > 0.5 {
            orientation - 0.5
        } else {
            orientation + 0.5
        } - 0.5; // Dans la plage [-0.5, 0.5]

        let is_aligned = rel_orientation.abs() < ANGLE_TOLERANCE;

        // À courte distance, être moins strict sur l'alignement
        let close_range_bonus = distance < 120.0;

        return is_in_range && (is_aligned || close_range_bonus);
    }

    // Calcule la position optimale du canon en fonction de l'orientation de l'ennemi
    fn calculate_gun_traversal(&self, orientation: f32) -> f32 {
        // Le canon peut être réglé de 0 à 1
        // On mappe l'orientation de l'ennemi à la traverse du canon

        // Normaliser l'orientation pour qu'elle soit dans [-0.5, 0.5]
        // où 0 représente l'ennemi exactement devant
        let normalized_orientation = if orientation > 0.5 {
            orientation - 0.5
        } else {
            orientation + 0.5
        } - 0.5;

        // Calculer la traverse de base
        // L'ennemi directement devant (orientation=0.5) -> traverse=0.5
        // L'ennemi à gauche (orientation=0.25) -> traverse plus proche de 0
        // L'ennemi à droite (orientation=0.75) -> traverse plus proche de 1

        // La fonction est centrée sur 0.5 (canon vers l'avant)
        // et va de 0 (complètement à gauche) à 1 (complètement à droite)
        let traverse_base = 0.5 + (normalized_orientation * 0.8);

        // Limiter la traverse entre 0 et 1
        traverse_base.max(0.0).min(1.0)
    }    fn get_closest_enemy_fast(&mut self) -> Option<(f32, f32)> {
        // Nettoyage du buffer avant d'envoyer la requête
        let _ = self.connection.try_clone().unwrap().set_nonblocking(true);
        let mut discard_buffer = [0; 1024];
        let _ = self.connection.try_clone().unwrap().read(&mut discard_buffer);
        let _ = self.connection.try_clone().unwrap().set_nonblocking(false);

        // Envoi de la requête avec un retour à la ligne explicite
        if let Err(_) = self.connection.write_all("CBOT\n".as_bytes()) {
            return None;
        }

        // Assurer que la commande est envoyée immédiatement
        let _ = self.connection.flush();

        // Attendre un court instant pour la réponse
        thread::sleep(Duration::from_millis(10));

        // Configuration pour lecture avec timeout
        let _ = self.connection.try_clone().unwrap().set_read_timeout(Some(Duration::from_millis(50)));

        // Lecture de la réponse
        let mut buffer = [0; 1024];
        match self.connection.try_clone().unwrap().read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);

                // Parse la réponse en cherchant le format spécifique "orientation/distance"
                for line in response.lines() {
                    if line.contains("/") && !line.contains("EMPTY") {
                        let parts: Vec<&str> = line.trim().split('/').collect();
                        if parts.len() == 2 {
                            if let (Ok(orientation), Ok(distance)) =
                                (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                                return Some((orientation, distance));
                            }
                        }
                    }
                }
            },
            _ => {}
        }

        None
    }

    // Méthode améliorée pour un mouvement plus réactif
    fn move_towards_enhanced(&mut self, orientation: f32, distance: f32) {
        // L'orientation est dans [0,1]
        // 0.5 représente l'ennemi devant, 0.25 à gauche, 0.75 à droite, 0 ou 1 derrière

        // Normalisation de l'orientation pour faciliter le calcul
        let normalized_orientation = if orientation > 0.5 {
            orientation - 0.5
        } else {
            orientation + 0.5
        } - 0.5; // Maintenant dans [-0.5, 0.5]

        // Calcul de l'intensité de virage nécessaire - plus proche de 0 = tout droit
        let turn_intensity = normalized_orientation * 2.0; // [-1.0, 1.0]

        // Puissance de base plus élevée pour un déplacement plus rapide
        let base_power = 0.8;

        // Règle spéciale pour les ennemis derrière (orientation proche de 0 ou 1)
        let is_behind = orientation < 0.1 || orientation > 0.9;

        if is_behind {
            // Tourner rapidement pour faire face à l'ennemi
            if orientation < 0.1 {
                // Ennemi à l'arrière-gauche - tourne à gauche
                self.mot_l(0.3);
                self.mot_r(0.8);
            } else {
                // Ennemi à l'arrière-droite - tourne à droite
                self.mot_l(0.8);
                self.mot_r(0.3);
            }
        } else if turn_intensity.abs() < 0.1 {
            // Si l'ennemi est presque devant, avancer tout droit rapidement
            self.mot_l(base_power + 0.15);
            self.mot_r(base_power + 0.15);
        } else {
            // Adaptation des moteurs en fonction de l'orientation
            // L'intensité du virage est plus forte quand l'ennemi est loin de l'axe
            let turn_factor = turn_intensity.abs() * 1.2; // Amplification du virage

            if turn_intensity < 0.0 {
                // Ennemi à gauche, tourne à gauche
                let left_power = base_power - turn_factor;
                let right_power = base_power + (turn_factor * 0.5); // Boost léger au moteur opposé
                self.mot_l(left_power.max(0.2));
                self.mot_r(right_power.min(1.0));
            } else {
                // Ennemi à droite, tourne à droite
                let right_power = base_power - turn_factor;
                let left_power = base_power + (turn_factor * 0.5); // Boost léger au moteur opposé
                self.mot_l(left_power.min(1.0));
                self.mot_r(right_power.max(0.2));
            }
        }
    }

    // Méthode améliorée pour une recherche plus efficace
    fn rotate_search(&mut self) {
        static mut SEARCH_DIRECTION: bool = true;
        static mut SEARCH_TIME: u64 = 0;

        unsafe {
            SEARCH_TIME += 1;

            // Changer de direction de rotation toutes les ~3 secondes
            if SEARCH_TIME > 150 { // 150 * 20ms = 3s
                SEARCH_DIRECTION = !SEARCH_DIRECTION;
                SEARCH_TIME = 0;
            }

            if SEARCH_DIRECTION {
                // Tourner à gauche
                self.mot_l(0.3);
                self.mot_r(0.7);
            } else {
                // Tourner à droite
                self.mot_l(0.7);
                self.mot_r(0.3);
            }
        }
    }

    // Méthode pour arrêter le suivi d'ennemi
    pub fn stop_tracking(&self) {
        self.stop_moving();
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

        // Teste le démarrage
        bot.start_moving();
        thread::sleep(Duration::from_secs(1)); // Laisse tourner un peu

        // Teste l'arrêt
        bot.stop_moving();
        thread::sleep(Duration::from_secs(1)); // S'assure que le thread s'est arrêté
    }
}