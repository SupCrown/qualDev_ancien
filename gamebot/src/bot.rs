use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::{Duration, Instant};
use std::f32::consts::PI;
use std::thread;

pub struct Bot {
    stream: TcpStream,
    last_projectile_distance: Option<f32>,
    last_heartbeat: Instant,
    running: bool,
}

impl Bot {
    pub fn new(mut stream: TcpStream) -> Self {
        // Désactiver Nagle pour envoyer les commandes immédiatement
        stream
            .set_nodelay(true)
            .expect("Échec de la configuration nodelay");
        Bot {
            stream,
            last_projectile_distance: None,
            last_heartbeat: Instant::now(),
            running: true,
        }
    }

    /// Méthode pour envoyer une commande sans attendre de réponse
    fn send_command_no_reply(&mut self, command: &str) {
        println!("[SEND] {}", command);
        if let Err(e) = self.stream.write_all(command.as_bytes()) {
            eprintln!("Erreur d'écriture: {}", e);
            return;
        }
        if let Err(e) = self.stream.flush() {
            eprintln!("Erreur de flush: {}", e);
            return;
        }
    }

    /// Méthode pour envoyer une commande et attendre une réponse (avec timeout)
    fn send_command_with_reply(&mut self, command: &str) -> String {
        println!("[SEND] {}", command);
        if let Err(e) = self.stream.write_all(command.as_bytes()) {
            eprintln!("Erreur d'écriture: {}", e);
            return String::new();
        }
        if let Err(e) = self.stream.flush() {
            eprintln!("Erreur de flush: {}", e);
            return String::new();
        }
        // Configurer un timeout court pour la lecture
        self.stream
            .set_read_timeout(Some(Duration::from_millis(30)))
            .expect("Erreur lors du réglage du timeout");

        let mut buffer = [0; 1024];
        let start = Instant::now();
        let response = loop {
            match self.stream.read(&mut buffer) {
                Ok(n) if n > 0 => break String::from_utf8_lossy(&buffer[..n]).to_string(),
                Ok(_) => {
                    if start.elapsed() > Duration::from_millis(30) {
                        break String::new();
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
                    if start.elapsed() > Duration::from_millis(30) {
                        break String::new();
                    }
                }
                Err(e) => {
                    eprintln!("Erreur de lecture: {}", e);
                    break String::new();
                }
            }
        };

        if response.is_empty() {
            println!("[RECV] (aucune donnée reçue dans le délai)");
        } else {
            println!("[RECV] {}", response);
        }
        response
    }

    /// Envoie le heartbeat si plus de 2 secondes se sont écoulées
    fn maybe_heartbeat(&mut self) {
        if self.last_heartbeat.elapsed() >= Duration::from_secs(2) {
            let heartbeat = "LIVE#";
            println!("[SEND][HEARTBEAT] {}", heartbeat);
            if let Err(e) = self.stream.write_all(heartbeat.as_bytes()) {
                eprintln!("Erreur d'écriture heartbeat: {}", e);
            }
            if let Err(e) = self.stream.flush() {
                eprintln!("Erreur lors de la flush heartbeat: {}", e);
            }
            self.last_heartbeat = Instant::now();
        }
    }

    /// Exemple pour récupérer les informations sur le bot ennemi
    fn get_closest_enemy(&mut self) -> Option<(f32, f32)> {
        let response = self.send_command_with_reply("CBOT#");
        if response.is_empty() {
            return None;
        }
        let parts: Vec<&str> = response.trim().split('/').collect();
        if parts.len() != 2 {
            return None;
        }
        match (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
            (Ok(orientation), Ok(distance)) => Some((orientation, distance)),
            _ => None,
        }
    }

    /// Exemple pour récupérer les informations sur le projectile le plus proche
    fn get_closest_projectile(&mut self) -> Option<(f32, f32)> {
        let response = self.send_command_with_reply("CPROJ#");
        if response.is_empty() {
            return None;
        }
        let parts: Vec<&str> = response.trim().split('/').collect();
        if parts.len() != 2 {
            return None;
        }
        match (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
            (Ok(orientation), Ok(distance)) => Some((orientation, distance)),
            _ => None,
        }
    }

    /// Avance le bot en fonction d'une orientation relative et d'une distance estimée
    fn move_towards(&mut self, relative_orientation: f32, distance: f32) {
        // Calcul de l'angle relatif (0.5 = devant, 0.25 = gauche, 0.75 = droite)
        let relative_angle = (relative_orientation - 0.5) * 2.0 * PI;
        let base_speed = 0.8;
        let turn_factor = 0.3 * relative_angle.sin();
        let distance_factor = if distance < 100.0 { 0.5 } else { 1.0 };

        let left_motor = ((base_speed + turn_factor) * distance_factor)
            .max(0.0)
            .min(1.0);
        let right_motor = ((base_speed - turn_factor) * distance_factor)
            .max(0.0)
            .min(1.0);

        self.send_command_no_reply(&format!("MotL={}#", left_motor));
        self.send_command_no_reply(&format!("MotR={}#", right_motor));
    }

    /// Oriente le canon du bot
    fn aim_gun(&mut self, relative_orientation: f32) {
        self.send_command_no_reply(&format!("GunTrav={}#", relative_orientation));
    }

    /// Tente de tirer
    fn shoot(&mut self, should_shoot: bool) {
        let trigger_value = if should_shoot { 1.0 } else { 0.0 };
        self.send_command_no_reply(&format!("GunTrig={}#", trigger_value));
    }

    /// Action d'évitement en cas de projectile dangereux
    fn evade_projectile(&mut self, projectile_orientation: f32) {
        let evasive_angle = if projectile_orientation < 0.5 { 0.75 } else { 0.25 };

        if evasive_angle < 0.5 {
            self.send_command_no_reply("MotL=0.0#");
            self.send_command_no_reply("MotR=1.0#");
        } else {
            self.send_command_no_reply("MotL=1.0#");
            self.send_command_no_reply("MotR=0.0#");
        }
        self.send_command_no_reply("MSG=Evading projectile!#");
    }

    fn is_projectile_approaching(&mut self, current_distance: f32) -> bool {
        if let Some(last_distance) = self.last_projectile_distance {
            current_distance < last_distance
        } else {
            false
        }
    }

    /// La boucle principale du bot dans un seul thread
    pub fn run(&mut self) {
        // Envoi initial de commandes de configuration
        self.send_command_no_reply("NAME=RustBot#");
        self.send_command_no_reply("COL=FF0000#");
        println!("Bot 'RustBot' initialisé et prêt au combat!");

        while self.running {
            // Vérifier et envoyer le heartbeat si besoin
            self.maybe_heartbeat();

            // Vérifier la présence d'un projectile
            if let Some((proj_orientation, proj_distance)) = self.get_closest_projectile() {
                let is_approaching = self.is_projectile_approaching(proj_distance);
                self.last_projectile_distance = Some(proj_distance);

                if is_approaching && proj_distance < 100.0 {
                    self.evade_projectile(proj_orientation);
                    // On attend un court instant pour laisser le temps à l'évitement de se faire
                    thread::sleep(Duration::from_millis(300));
                    continue; // Passe à l'itération suivante
                }
            } else {
                self.last_projectile_distance = None;
            }

            // Rechercher et engager un ennemi
            if let Some((enemy_orientation, enemy_distance)) = self.get_closest_enemy() {
                self.aim_gun(enemy_orientation);
                self.move_towards(enemy_orientation, enemy_distance);

                // Déclencher le tir si bien aligné (< 0.05 d'écart par rapport à 0.5) et à moins de 300.0
                let is_aligned = (enemy_orientation - 0.5).abs() < 0.05;
                self.shoot(is_aligned && enemy_distance < 300.0);
                self.send_command_no_reply(&format!("MSG=Target acquired: {:.1}m#", enemy_distance));
            } else {
                // Mode "search" : avancer en tournant légèrement pour chercher la cible
                self.send_command_no_reply("MotL=0.7#");
                self.send_command_no_reply("MotR=0.3#");
                self.shoot(false);
                self.send_command_no_reply("MSG=Searching...#");
            }

            // Petite pause pour éviter de saturer le CPU
            thread::sleep(Duration::from_millis(50));
        }
        println!("La boucle principale du bot est arrêtée.");
    }

    /// Méthode pour arrêter le bot proprement
    pub fn shutdown(&mut self) {
        self.running = false;
        let _ = self.stream.shutdown(Shutdown::Both);
        println!("Arrêt du bot demandé.");
    }
}
