use std::env;

mod mp3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    mp3::get_mp3_duration(&file_path)
}
