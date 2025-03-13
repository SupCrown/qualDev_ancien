use clap::Parser;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use gamebot::bot::Bot;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
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

    // Connexion au serveur
    let connection = TcpStream::connect(&server_address).unwrap();
    let mut bot = Bot::new(connection, "fabrice".to_string());

    // Démarrage du bot
    bot.track_and_shoot_enemy(); // Démarre la logique de poursuite et de tir
    thread::sleep(Duration::from_secs(1000)); // Attend 1000 secondes
    bot.stop_tracking(); // Arrête la poursuite
}