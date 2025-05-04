use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename: String = args[1].clone();
        let query: String = args[2].clone();

        Config { filename, query }
    }
}

pub fn run(config: Config) {
    let content: String = fs::read_to_string(config.filename).expect("Ocurrió un error");

    let results: Vec<&str> = search(&config.query, &content);

    if !results.is_empty() {
        show_results(results);
    } else {
        println!("No hay resultados con la búsqueda: {}", config.query);
    }
}

fn show_results(results: Vec<&str>) {
    for result in results.iter() {
        println!("{}", result)
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
