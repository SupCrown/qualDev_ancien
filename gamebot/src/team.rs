use clap::Parser;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use gamebot::bot::Bot;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Nombre de bots dans l'équipe
    #[clap(short, long, default_value = "2")]
    num_bots: usize,

    /// Adresse IP du serveur
    #[clap(short = 'a', long, default_value = "172.17.0.1")]
    server_address: String,

    /// Port du serveur
    #[clap(short = 'p', long, default_value = "6969")]
    server_port: u16,
}

fn main() {
    let args = Args::parse();
    let server_address = format!("{}:{}", args.server_address, args.server_port);

    let mut handles = vec![];
    let team_name = "TeamAlpha";

    for i in 0..args.num_bots {
        let bot_name = format!("{}-{}", team_name, i + 1); // Nom du bot avec préfixe d'équipe
        let server_address = server_address.clone();

        let handle = thread::spawn(move || {
            // Connexion au serveur
            let connection = TcpStream::connect(&server_address).unwrap();
            let mut bot = Bot::new(connection, bot_name);

            // Démarrage du bot
            bot.track_and_shoot_enemy(); // Démarre la logique de poursuite et de tir
            thread::sleep(Duration::from_secs(1000)); // Attend 1000 secondes
            bot.stop_tracking(); // Arrête la poursuite
        });

        handles.push(handle);
    }

    // Attendre que tous les threads se terminent
    for handle in handles {
        handle.join().unwrap();
    }
}