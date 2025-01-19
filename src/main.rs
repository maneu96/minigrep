use std::process;
use std::env;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    //println!("Searching for {} in file {}", conf.query ,conf.file_path);

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


