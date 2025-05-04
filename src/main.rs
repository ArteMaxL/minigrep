use std::env;

use minigrep::Config;

fn main() {
    // Example: cargo run src/log.txt Exception
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let config = Config::new(&args);

        println!("archivo: {}, query: {}", config.filename, config.query);

        minigrep::run(config);
    }
}
