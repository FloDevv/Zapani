// random.rs
use rand::seq::SliceRandom;
use std::path::PathBuf;

pub fn customrandom(files: &mut Vec<PathBuf>) {
    shuffle_files(files);
}

fn shuffle_files(files: &mut Vec<PathBuf>) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    files.shuffle(&mut rng);
}
