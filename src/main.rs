use std::env;

mod mp3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let audio = mp3::init_audio(&file_path);
    println!("{:?}", audio.duration)
}
