use sfml::{
    audio::{Music}
};
use sfml::audio::SoundSource;

struct MusicTrack {
    filename: String,
    volume: i8
}

pub struct MusicPlayer {
    tracks: Vec<MusicTrack>,
    test: Option<sfml::audio::Music>
}

impl MusicPlayer {
    pub fn new() -> MusicPlayer {
        MusicPlayer {
            tracks: vec![],
            test: None
        }
    }

    pub fn play_music(&mut self, filename: &str) {
        match sfml::audio::Music::from_file(filename) {
            Some(mut music) => {
                music.play();
                self.test = Option::Some(music);
            }
            None => {self.test = None}
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        match self.test {
            Some(ref mut music) => {
                music.set_volume(volume);
            }
            None => {}
        }
    }

    pub fn add_track(&mut self, filename: String) {
        self.tracks.push(MusicTrack{
            filename,
            volume: 100
        });
    }

    pub fn get_tracks(&self) -> usize {
        self.tracks.len()
    }
}