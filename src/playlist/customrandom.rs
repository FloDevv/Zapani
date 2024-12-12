// random.rs
use rand::{ seq::SliceRandom, thread_rng };
use std::path::PathBuf;

pub fn customrandom(files: &mut Vec<PathBuf>) {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    files.shuffle(&mut rng);

    let len: usize = files.len();
    for i in 1..len {
        if files[i].file_name() == files[i - 1].file_name() {
            for j in i + 1..len {
                if files[j].file_name() != files[i - 1].file_name() {
                    files.swap(i, j);
                    break;
                }
            }
        }
    }
}
