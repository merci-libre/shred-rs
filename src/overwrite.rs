use rand::{self, Rng};
use std::fs::{File, Metadata};
use std::io::{Seek, SeekFrom, Write};
use std::sync::mpsc;
use std::thread;

fn fill_vector(mut vec: Vec<u8>, size: usize, opt: u8) -> Vec<u8> {
    let mut q = rand::rng();
    while vec.len() != size {
        let x: u8;
        match opt {
            0 => x = q.random_range(0..255),
            1 => x = 0,
            _ => todo!(),
        }
        vec.push(x);
    }
    return vec;
}
fn rand_overwrite(size: usize, mut file: &File, opt: u8) {
    let (tx, rx) = mpsc::channel();
    let (dtx, drx) = mpsc::channel();
    let vec: Vec<u8> = Vec::new();
    tx.send(vec).unwrap();
    let handle = thread::spawn(move || {
        let mut data = rx.recv().unwrap();
        data = fill_vector(data, size, opt);
        dtx.send(data).unwrap();
    });
    handle.join().unwrap();
    let content = drx.recv().unwrap();
    file.write_all(content.as_slice()).unwrap();
}

pub fn large_overwrite(mut file: &File, file_metadata: &Metadata, zero: bool) {
    let mut start_iterations = 4096;
    let goal = 0;
    let mut opt = 0;
    if zero {
        opt = 1;
    }

    file.seek(SeekFrom::Start(0)).unwrap();
    if file_metadata.len() > 4096 {
        let m = (file_metadata.len() / 4096) + 1;
        start_iterations *= m;
    }
    while start_iterations != goal {
        rand_overwrite(128, file, opt);
        start_iterations -= 128;
    }
}

pub fn sectioned_overwrite(size: usize, mut file: &File) {
    file.seek(SeekFrom::Start(0)).unwrap();
    rand_overwrite(size, file, 0);
}
