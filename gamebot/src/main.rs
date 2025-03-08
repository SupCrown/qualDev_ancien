mod bot;

use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crate::bot::Bot;

fn main() {


    let connection = TcpStream::connect("172.17.0.1:6969").unwrap();
    let mut bot = Bot::new(connection, "fabrice".to_string());

    // Démarre le thread de déplacement
    bot.track_and_shoot_enemy();

    // Laisse le bot bouger pendant 3 secondes
    thread::sleep(Duration::from_secs(1000));

    // Stoppe le déplacement
    bot.stop_tracking();

}