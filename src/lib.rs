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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let args = vec![
            String::from("minigrep"),
            String::from("archivo.txt"),
            String::from("query"),
        ];
        let config = Config::new(&args);

        assert_eq!(config.filename, "archivo.txt");
        assert_eq!(config.query, "query");
    }

    #[test]
    fn test_search_with_results() {
        let query = "Rust";
        let content = "\
Rust es genial.
Aprender Rust es divertido.
El lenguaje Rust es rápido.";

        let results = search(query, content);

        assert_eq!(results, vec![
            "Rust es genial.",
            "Aprender Rust es divertido.",
            "El lenguaje Rust es rápido."
        ]);
    }

    #[test]
    fn test_search_no_results() {
        let query = "Python";
        let content = "\
            Rust es genial.
            Aprender Rust es divertido.";

        let results = search(query, content);

        assert!(results.is_empty());
    }

    #[test]
    fn test_search_partial_match() {
        let query = "genial";
        let content = "Rust es genial.";

        let results = search(query, content);

        assert_eq!(results, vec!["Rust es genial."]);
    }
}