use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err_message| {
        println!("Problem parsing arguments: {}", err_message);
        process::exit(1);
    });
    if let Err(err_message) = minigrep::run(config) {
        println!("Application error:{}",err_message);
        process::exit(1);
    }
}


