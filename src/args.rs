use clap::Parser;
#[derive(Parser, Debug)]
pub struct ShredArgs {
    #[arg(long, short = 'n', default_value = "3")]
    /// Determines how many iterations the file is
    /// put through the software.
    pub iterations: u64,
    #[arg(long, short, default_value = "0")]
    /// Shred this many bytes from the file. If no value is passed, the entire file will be
    /// shredded by default. (The represented value of '0' is a placeholder for the size of the entire file.)
    pub size: usize,
    #[arg(long, short)]
    /// Show progress of the program.
    pub verbose: bool,
    #[arg(long, short)]
    /// Add a final overwrite of zeros to hide shredding
    pub zero: bool,
    #[arg(short = 'u')]
    ///Remove File After Shredding.
    pub delete: bool,
    /// Overwrites specified files repeatedly with random bytes, making a recovery process for the
    /// desired data much more expensive and costly for an attacker.
    pub file: Vec<String>,
}
