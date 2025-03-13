use clap::Parser;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use gamebot::bot::Bot;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of bots in the team
    #[clap(short, long, default_value = "2")]
    num_bots: usize,
}

fn main() {
    let args = Args::parse();

    let mut handles = vec![];
    let team_name = "TeamAlpha";
    let server_address = "172.17.0.1:6969";

    for i in 0..args.num_bots {
        let bot_name = format!("TeamAlpha-{}", i + 1); // Ajout du préfixe "TeamAlpha-"
        let server_address = server_address.to_string();

        let handle = thread::spawn(move || {
            let connection = TcpStream::connect(&server_address).unwrap();
            let mut bot = Bot::new(connection, bot_name);

            bot.track_and_shoot_enemy();
            thread::sleep(Duration::from_secs(1000));
            bot.stop_tracking();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}