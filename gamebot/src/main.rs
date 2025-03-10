use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Write, BufRead, BufReader};
use std::net::TcpStream;
use rand::Rng;
use serde_json::Value;
use std::env;

// Actions possibles
const ACTIONS: [&str; 4] = ["MOVE_LEFT", "MOVE_RIGHT", "ADVANCE", "SHOOT"];

// Structure pour la sauvegarde de la Q-table
// QTable structure (avec Map sérialisable)
#[derive(Serialize, Deserialize)]
struct SerializableQTable {
    data: HashMap<String, [f32; 4]>,
}

// Fonction principale
fn main() {
    // Lire les arguments de la ligne de commande
    let (server_address, bot_name) = get_arguments();

    // Connexion au serveur
    let mut connection = TcpStream::connect(&server_address)
        .unwrap_or_else(|_| panic!("Impossible de se connecter au serveur : {}", server_address));

    // Identification du bot
    send_command(&mut connection, &format!("NAME={}", bot_name));
    send_command(&mut connection, "COL=255=0=0");

    // Charger la Q-table si elle existe
    let mut q_table = load_q_table("q_table.json");

    // Paramètres du Q-learning
    let learning_rate = 0.2;
    let discount_factor = 0.9;
    let epsilon = 0.6; // Probabilité d'explorer

    // Boucle principale
    let mut last_orient: Option<f32> = None;       // Stocker l'orientation précédente du bot
    let mut stuck_counter = 0;                    // Compteur bloqué
    let stuck_threshold = 15;                     // Nombre de cycles avant de juger le bot bloqué

    loop {
        // Obtenir les observations actuelles
        let (enemy_orientation, enemy_distance) = get_cbot_observation(&mut connection);
        let (proj_orientation, proj_distance) = get_cproj_observation(&mut connection);

        // Calculer l'orientation actuelle
        let current_orient = get_orient(&mut connection);

        // Discrétiser l'état actuel
        let state = discretize_state(enemy_orientation, enemy_distance, proj_orientation, proj_distance);

        // Choisir l'action à effectuer selon epsilon-greedy
        let action = select_action_epsilon_greedy(state, &q_table, epsilon);

        // Exécuter l'action choisie
        match action {
            0 => send_command(&mut connection, "MotL=0.4#MotR=0.88"),   // MOVE_LEFT
            1 => send_command(&mut connection, "MotL=0.88#MotR=0.4"),   // MOVE_RIGHT
            2 => send_command(&mut connection, "MotL=1#MotR=1"),     // ADVANCE
            3 => send_command(&mut connection, "GunTrig=1"),         // SHOOT
            _ => {},
        }

        // Requêtes environnementales après mise à jour
        let (new_enemy_orientation, new_enemy_distance) = get_cbot_observation(&mut connection);
        let (new_proj_orientation, new_proj_distance) = get_cproj_observation(&mut connection);

        // Calculer le prochain état (nouvelle position résultant de l'action effectuée)
        let next_state = discretize_state(new_enemy_orientation, new_enemy_distance, new_proj_orientation, new_proj_distance);

        let mut bot_is_stuck = false;
        if let Some(last) = last_orient {
            // Si l'orientation n'a pas changé significativement
            if (current_orient - last).abs() < 0.01 {
                bot_is_stuck = true;
            }
        }
        // Calculer la récompense associée à l'action effectuée
        let reward = calculate_reward(
            enemy_distance,
            enemy_orientation,
            proj_distance,
            action,
            &mut stuck_counter,
            stuck_threshold,
            bot_is_stuck,
        );

        // Mettre à jour la Q-table avec le nouvel état
        update_q_table(&mut q_table, state, action, reward, next_state, learning_rate, discount_factor);

        // Sauvegarder périodiquement la Q-table
        save_q_table(&q_table, "q_table_020906.json");

    }
}

fn get_arguments() -> (String, String) {
    // Obtenir les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();

    // Valeur par défaut pour l'adresse IP
    let default_ip = "172.17.0.1:6969".to_string();

    // Vérifier si le pseudo est fourni
    if args.len() < 2 {
        eprintln!("Erreur : Vous devez fournir un pseudo !");
        eprintln!("Utilisation : {} <pseudo> [adresse_ip]", args[0]);
        std::process::exit(1); // Quitter le programme avec un code d'erreur
    }

    // Le pseudo est obligatoire (2e argument)
    let name = args[1].clone();

    // L'adresse IP est facultative (3e argument)
    let ip = args.get(2).unwrap_or(&default_ip).clone();

    (ip, name)
}


// Fonction pour envoyer une commande au serveur
fn send_command(connection: &mut TcpStream, command: &str) {
    connection
        .write_all(format!("{}\n", command).as_bytes())
        .expect("Impossible d'envoyer la commande au serveur !");
}


fn get_orient(connection: &mut TcpStream) -> f32 {
    send_command(connection, "ORIENT");
    let response = read_response(connection);
    response.parse().unwrap_or(0.0)

}

// Récupérer les données de l'ennemi (CBOT)
fn get_cbot_observation(connection: &mut TcpStream) -> (f32, f32) {
    send_command(connection, "CBOT");
    let response = read_response(connection);
    let parts: Vec<&str> = response.split('/').collect();
    if parts.len() == 2 {
        let orientation: f32 = parts[0].parse().unwrap_or(0.0);
        let distance: f32 = parts[1].parse().unwrap_or(0.0);
        return (orientation, distance);
    }
    (0.0, 0.0)
}

// Récupérer les données des projectiles (CPROJ)
fn get_cproj_observation(connection: &mut TcpStream) -> (f32, f32) {
    send_command(connection, "CPROJ");
    let response = read_response(connection);
    let parts: Vec<&str> = response.split('/').collect();
    if parts.len() == 2 {
        let orientation: f32 = parts[0].parse().unwrap_or(0.0);
        let distance: f32 = parts[1].parse().unwrap_or(0.0);
        return (orientation, distance);
    }
    (0.0, 0.0)
}

fn is_bot_stuck(
    current_orient: f32,
    last_orient: Option<f32>,
    motors_active: bool,
    stuck_counter: &mut usize,
    stuck_threshold: usize
) -> bool {
    // Critère 1 : l'orientation ne change pas quand le bot est censé bouger
    let orient_unchanged = last_orient.map_or(false, |last| (current_orient - last).abs() < 0.01);

    // Critère 2 : les moteurs sont activés
    if motors_active && orient_unchanged {
        *stuck_counter += 1;
        if *stuck_counter >= stuck_threshold {
            println!("Bot is stuck! Counter exceeded threshold.");
            return true;
        }
    } else {
        // Si le bot commence à bouger à nouveau
        *stuck_counter = 0;
    }

    false
}

// Lire les réponses du serveur
fn read_response(connection: &mut TcpStream) -> String {
    let mut reader = BufReader::new(connection.try_clone().unwrap());
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    response.trim().to_string()
}

// Discrétisation des observations
fn discretize_state(enemy_orientation: f32, enemy_distance: f32, proj_orientation: f32, proj_distance: f32) -> (usize, usize, usize, usize) {
    let enemy_orientation_bin = (enemy_orientation * 10.0).floor() as usize; // Réduit à 5 buckets (0.0 -> 1.0 devient 5 intervalles)
    let enemy_distance_bin = (enemy_distance / 15.0).floor() as usize;      // Divise les distances en grands intervalles (30 unités par bucket)
    let proj_orientation_bin = (proj_orientation * 10.0).floor() as usize;  // Similaire à l'orientation ennemie
    let proj_distance_bin = (proj_distance / 30.0).floor() as usize;       // Pareil pour les projectiles

    // Retourne les valeurs disrétisées comme un état unique
    (enemy_orientation_bin, enemy_distance_bin, proj_orientation_bin, proj_distance_bin)
}

// Sélectionner une action (epsilon-greedy)
fn select_action_epsilon_greedy(
    state: (usize, usize, usize, usize),
    q_table: &HashMap<(usize, usize, usize, usize), [f32; 4]>,
    epsilon: f32,
) -> usize {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(epsilon as f64) { // Exploration
        rng.gen_range(0..ACTIONS.len())
    } else { // Exploitation
        if let Some(q_values) = q_table.get(&state) {
            q_values
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0
        } else {
            rng.gen_range(0..ACTIONS.len()) // Pas encore d'entrée pour cet état : choisir aléatoirement
        }
    }
}

fn calculate_reward(
    enemy_distance: f32,
    enemy_orientation: f32,
    proj_distance: f32,
    action: usize,
    stuck_counter: &mut usize,
    stuck_threshold: usize,
    bot_is_stuck: bool,
) -> f32 {
    let mut reward = 0.0;

    // Récompense : tir aligné avec l'ennemi
    if action == 3 && enemy_distance < 30.0 && (enemy_orientation - 0.5).abs() < 0.02 {
        reward += 10.0; // Récompense pour un tir aligné
        println!("Récompense : tir précis aligné (+10)");
    }

    // Malus : tir gaspillé lorsqu'aucun ennemi n'est aligné
    /*a voir
    if action == 3 && (enemy_orientation < 0.3 || enemy_orientation > 0.7) {
        reward -= 5.0; // Malus pour tir inefficace
        println!("Malus : tir gaspillé (-5)");
    }*/

    // Récompense : réduction de la distance à l'ennemi
    if enemy_distance < 20.0 {
        reward += 3.0; // Récompense pour s'approcher de l'ennemi
        //println!("Récompense : réduction de distance à l'ennemi (+2)");
    }

    // Malus : distance aux projectiles dangereux réduite
    /*ses propres projectiles comptent
    if proj_distance < 15.0 {
        reward -= 5.0; // Malus pour danger d'un projectile
        println!("Malus : proximité d'un projectile dangereux (-5)");
    }*/

    // Malus : bot bloqué
    if bot_is_stuck {
        *stuck_counter += 1;
        if *stuck_counter >= stuck_threshold {
            reward -= 10.0; // Malus pour rester bloqué
            println!("Malus : bot bloqué trop longtemps (-10)");
        }
    } else {
        *stuck_counter = 0; // Réinitialiser le compteur si le bot bouge à nouveau
    }

    // Récompense : survie à long terme en cycles
    /*if *stuck_counter % 50 == 0 {
    pas viable
        reward += 5.0;
        println!("Récompense : survie longue (+5)");
    }*/

    // Malus : actions inutiles ou stagnation
    if action == 2 && enemy_distance > 27.0 { // Action d'avancer avec ennemi éloigné
        reward -= 5.0; // Pénalité pour inefficacité
        println!("Malus : action inefficace (avance inutile, -5)");
    }

    reward
}

// Mise à jour de la Q-table
fn update_q_table(
    q_table: &mut HashMap<(usize, usize, usize, usize), [f32; 4]>,
    state: (usize, usize, usize, usize),  // État actuel
    action: usize,                        // Action choisie
    reward: f32,                          // Récompense immédiate donnée après l'action
    next_state: (usize, usize, usize, usize), // État atteint après l'action
    learning_rate: f32,
    discount_factor: f32,
) {
    // Récupérer les Q-valeurs actuelles pour le prochain état
    let next_q_values = q_table.entry(next_state).or_insert([0.0; 4]);
    let max_next_q = next_q_values.iter().cloned().fold(f32::NEG_INFINITY, f32::max); // Max pour l'état futur

    // Mettre à jour la Q-valeur pour le couple (state, action)
    let q_values = q_table.entry(state).or_insert([0.0; 4]);
    q_values[action] = q_values[action] + learning_rate * (reward + discount_factor * max_next_q - q_values[action]);
}

/// Sauvegarder la Q-table dans un fichier JSON
fn save_q_table(q_table: &HashMap<(usize, usize, usize, usize), [f32; 4]>, file_path: &str) {
    let serializable_map: HashMap<String, [f32; 4]> = q_table
        .iter()
        .map(|(k, v)| (serialize_key(k), *v)) // Convertit les clés en chaînes
        .collect();

    let table = SerializableQTable { data: serializable_map };
    let file = File::create(file_path).expect("Impossible de créer le fichier de sauvegarde.");
    serde_json::to_writer(file, &table).expect("Erreur lors de l'écriture de la Q-table.");
}

/// Charger la Q-table depuis un fichier JSON
fn load_q_table(file_path: &str) -> HashMap<(usize, usize, usize, usize), [f32; 4]> {
    // Vérifie si le fichier existe
    let file = File::open(file_path).ok();

    // Si le fichier est trouvé
    if let Some(file) = file {
        // Lecture et tentative de désérialisation
        let reader = BufReader::new(file);
        let value: Result<Value, _> = serde_json::from_reader(reader);

        match value {
            Ok(json_value) => {
                // Convertir la structure en HashMap
                if let Ok(table) = serde_json::from_value::<SerializableQTable>(json_value) {
                    return table
                        .data
                        .iter()
                        .map(|(k, v)| (deserialize_key(k), *v)) // Convertit les clés (chaînes) en tuples
                        .collect();
                } else {
                    println!("Le fichier JSON n'a pas de structure valide. Création d'une nouvelle Q-table.");
                }
            }
            Err(_) => {
                println!("Le fichier Q-table est corrompu ou vide. Création d'une nouvelle Q-table.");
            }
        }
    } else {
        // Si aucun fichier n'est trouvé
        println!("Aucun fichier de sauvegarde trouvé. Création d'une nouvelle Q-table.");
    }

    // Renvoie une Q-table vide si tout échoue
    HashMap::new()
}

/// Convertit un tuple en une chaîne de caractères
fn serialize_key(key: &(usize, usize, usize, usize)) -> String {
    format!("{},{},{},{}", key.0, key.1, key.2, key.3)
}

/// Convertit une chaîne en un tuple
fn deserialize_key(key: &str) -> (usize, usize, usize, usize) {
    let parts: Vec<usize> = key
        .split(',')
        .map(|part| part.parse::<usize>().expect("Clé invalide dans le fichier Q-table"))
        .collect();
    (parts[0], parts[1], parts[2], parts[3])
}