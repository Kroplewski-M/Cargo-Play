use std::{fs::File, sync::Arc};

use rodio::Decoder;

use crate::models::Track;

pub struct PlayerControl {
    pub current_track: Option<Track>,
    player: Arc<rodio::Player>,
    _sink: rodio::MixerDeviceSink, //must stay alive; dropping it stops all audio output
}
impl PlayerControl {
    pub fn new() -> Self {
        let _sink =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(_sink.mixer());

        Self {
            current_track: None,
            _sink,
            player: Arc::new(player),
        }
    }
    pub fn play_track(&mut self, track: &Track) {
        let cloned_player = self.player.clone();
        let location = track.location.clone();
        std::thread::spawn(move || {
            cloned_player.clear();
            let file = File::open(location).unwrap();
            // Decode that sound file into a source
            let source = Decoder::try_from(file).unwrap();
            // Play the sound directly on the device
            cloned_player.append(source);
            cloned_player.play();
        });
        self.current_track = Some(track.clone());
    }

    pub fn play_pause(&mut self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }
    pub fn adjust_volumne(&mut self, delta: f32) {
        let volume = (self.player.volume() + delta).clamp(0.0, 1.0);
        self.player.set_volume(volume);
    }
    pub fn volume(&self) -> f32 {
        self.player.volume()
    }
    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }
}
