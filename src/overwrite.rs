use rand::{self, Rng};
use std::fs::{File, Metadata};
use std::io::{Seek, SeekFrom, Write};
use std::sync::mpsc;
use std::thread;

fn fill_vector_random(mut vec: Vec<u8>, size: usize, opt: u8) -> Vec<u8> {
    let mut q = rand::rng();
    while vec.len() != size {
        let x: u8;
        match opt {
            0 => {
                x = q.random_range(0..255);
                vec = vec![x; size]
            }
            1 => {
                x = 0;
                vec = vec![x; size];
            }
            _ => todo!(),
        }
    }
    return vec;
}
fn fill_vector_pattern(mut vec: Vec<u8>, size: usize, opt: u8) {}

fn rand_overwrite(size: usize, mut file: &File, opt: u8) {
    let (tx, rx) = mpsc::channel();
    let (dtx, drx) = mpsc::channel();
    let vec: Vec<u8> = Vec::new();
    tx.send(vec).unwrap();
    //start a thread to start the shredding.
    let handle = thread::spawn(move || {
        let mut data = rx.recv().unwrap();
        data = fill_vector_random(data, size, opt);
        dtx.send(data).unwrap();
    });
    handle.join().unwrap();
    let content = drx.recv().unwrap();
    file.write_all(content.as_slice()).unwrap();
}

fn pattern_overwrite(size: usize, mut file: &File, opt: u8) {
    let (tx, rx) = mpsc::channel();
    let (dtx, drx) = mpsc::channel();
    let vec: Vec<u8> = Vec::new();
    tx.send(vec).unwrap();
    //start a thread to start the shredding.
    let handle = thread::spawn(move || {
        let mut data = rx.recv().unwrap();
        data = fill_vector_random(data, size, opt);
        dtx.send(data).unwrap();
    });
    handle.join().unwrap();
    let content = drx.recv().unwrap();
    file.write_all(content.as_slice()).unwrap();
}

pub fn large_overwrite(mut file: &File, file_metadata: &Metadata, zero: bool) {
    let mut start_iterations = 4096;
    let mut opt = 0;

    if zero {
        opt = 1;
    }

    file.seek(SeekFrom::Start(0)).unwrap();
    if file_metadata.len() > 4096 {
        let m = (file_metadata.len() / 4096) + 1;
        start_iterations *= m;
    }
    rand_overwrite(start_iterations as usize, file, opt);
}

pub fn sectioned_overwrite(size: usize, mut file: &File) {
    file.seek(SeekFrom::Start(0)).unwrap();
    rand_overwrite(size, file, 0);
}
