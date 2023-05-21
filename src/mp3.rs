use mp3_duration;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::fs::File;
use std::path::Path;
use std::time::Duration;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

#[allow(dead_code)]
fn run_sound() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec: AudioSpecDesired = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    // Start playback
    device.resume();

    std::thread::sleep(Duration::from_millis(5000));
}

#[allow(dead_code)]
pub fn get_mp3_duration(mp3_file: &str) {
    let path = Path::new(mp3_file);
    let file = File::open(path).unwrap();
    let duration = mp3_duration::from_file(&file).unwrap();
    println!("File duration: {:?}", duration);
}
