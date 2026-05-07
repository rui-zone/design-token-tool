mod cli;

/// Runs the command-line application and exits with an error code on failure.
fn main() {
    if let Err(error) = cli::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
