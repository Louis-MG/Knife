use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Path to the output file.
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
    /// Number of threads to use (number of file parallelised).
    #[arg(short, long)]
    thread: Option<u32>,
    /// Separation symbol for output file.
    #[arg(short, long)]
    sep: Option<String>,
    // std::path::PathBuf au lieu de Path parce que je veux garder le path et y avoir acces
    /// Path to the input files or file-of-file.
    #[arg(last = true)]
    input: Vec<std::path::PathBuf>,
}

fn main() {
    // attrape les argument comme vecteur de text
    let cli= Cli::parse();

    println!("Build succesful");
}
