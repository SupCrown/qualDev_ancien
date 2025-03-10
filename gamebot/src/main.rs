
mod bot;

use bot::Bot;
use std::net::TcpStream;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Demander l'adresse du serveur
    print!("Entrez l'adresse du serveur (par défaut 127.17.0.1:6969): ");
    io::stdout().flush()?;

    let mut address = String::new();
    io::stdin().read_line(&mut address)?;
    let address = address.trim();
    let address = if address.is_empty() { "127.17.0.1:6969" } else { address };

    // Demander le nom du bot
    print!("Entrez le nom du bot (par défaut RustBot): ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();
    let name = if name.is_empty() { "RustBot" } else { name };

    println!("Connexion au serveur {}...", address);

    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("Connecté! Initialisation du bot...");
            let mut bot = Bot::new(stream);

            // Initialiser le bot avec son nom et sa couleur (rouge vif)
            bot.initialize(name, (255, 0, 0));
            println!("Bot '{}' initialisé et prêt au combat!", name);

            // Lancer le bot
            bot.run();

            Ok(())
        },
        Err(e) => {
            eprintln!("Erreur de connexion au serveur: {}", e);
            Err(e)
        }
    }
}