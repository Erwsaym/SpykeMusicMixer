mod test;
mod audio;

use test::*;
use audio::*;

use std::io;

fn command(music_player: &mut MusicPlayer, input: &str) {
    let args : Vec<&str> = input.split(" ").collect();
    match args[0] {
        "volume" => {
            match args[1].parse::<f32>() {
                Ok(x) => {music_player.set_volume(x);}
                Err(e) => {println!("Error while giving volume arg");}
            }
        }
        _ => {}
    }
}

fn main() {
    let mut music_player = MusicPlayer::new();
    music_player.play_music("data/audio/music/astral.ogg");

    loop {
        println!(">");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                match input.trim() {
                    "exit" => { break; }
                    str => { command(&mut music_player, str); }
                }
            },
            Err(e) => {
                println!("Impossible to get command line : {}", e);
            }
        }

    }
}
