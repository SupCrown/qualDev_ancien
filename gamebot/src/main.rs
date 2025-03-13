use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use gamebot::bot::Bot;

fn main() {
    let connection = TcpStream::connect("172.17.0.1:6969").unwrap();
    let mut bot = Bot::new(connection, "fabrice".to_string());

    bot.track_and_shoot_enemy();
    thread::sleep(Duration::from_secs(1000));
    bot.stop_tracking();
}