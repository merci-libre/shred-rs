//implement modules
mod args;
mod overwrite;

use clap::Parser;
use std::env::current_dir;
use std::fs::{exists, remove_file, OpenOptions};
use std::process::exit;
use {args::*, overwrite::*};
fn main() {
    let args = ShredArgs::parse();
    //args= iterations, size, file, verbosity

    //implement a function to check errors?
    let filename = args.file.clone();
    let path = match current_dir() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    };
    let _exists = match exists(filename.clone()) {
        /*File does not exist*/
        Ok(false) => {
            eprintln!(
                "Error: {}/{filename} is either missing, or does not exist.",
                path.display()
            );
            exit(1)
        }
        /*Some unknown error*/
        Err(_e) => {
            eprintln!("something went wrong when searching for the file, exiting program.");
            exit(1);
        }
        /*File does exist*/
        Ok(true) => (),
    };

    shred(
        args.iterations,
        args.size,
        args.file,
        args.verbose,
        args.zero,
    )
    .unwrap();

    if args.delete {
        remove_file(filename).unwrap();
    }
}

fn shred(
    iterations: u64,
    size: usize,
    filename: String,
    verbose: bool,
    zero: bool,
) -> std::io::Result<()> {
    // all parameters
    // are arguments passed into the parser.

    let file = OpenOptions::new()
        .write(true)
        .open(filename.clone())
        .unwrap();

    let file_metadata = std::fs::metadata(filename.clone()).unwrap();

    // dbg!(&file_metadata);
    let mut count: u64 = 0;
    match size {
        // modules==> overwrite::*
        0 => {
            while count != iterations {
                if verbose {
                    eprintln!("shred: {filename}: pass {}/{iterations}...", count + 1);
                }
                large_overwrite(&file, &file_metadata, false);

                count += 1;
            }
        }
        _ => {
            while count != iterations {
                sectioned_overwrite(size, &file);
                if verbose {
                    eprintln!("shred: {filename}: {count}/{iterations}...");
                }
                count += 1;
            }
        }
    }
    if zero {
        large_overwrite(&file, &file_metadata, zero);
    }

    Ok(())
}
