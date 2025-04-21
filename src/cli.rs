use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    /// Paths to directories or files to read
    pub paths: Vec<PathBuf>,

    /// Maximum recursion depth for directories
    #[clap(short, long, default_value = "10")]
    pub max_depth: usize,

    /// Include error messages in the output
    #[clap(short, long)]
    pub include_errors: bool,

    /// Get number of characters and words of output (useful if output could be too long)
    #[clap(short, long)]
    pub output_information: bool,
    
    /// Copy the output to the system clipboard instead of printing it
    #[clap(short, long)]
    pub copy: bool,
}