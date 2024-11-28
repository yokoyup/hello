use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn parse_args(args: &[String]) -> (String, String, String) {
    let mut message = "Hello, World!".to_string();
    let mut name = "".to_string();
    let mut language = "en".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-m" | "--message" => {
                if i + 1 < args.len() {
                    message = args[i + 1].clone();
                    i += 1;
                } else {
                    println!("Error: Missing value for -m/--message");
                    return (message, name, language);
                }
            }
            "-n" | "--name" => {
                if i + 1 < args.len() {
                    name = args[i + 1].clone();
                    i += 1;
                } else {
                    println!("Error: Missing value for -n/--name");
                    return (message, name, language);
                }
            }
            "-l" | "--language" => {
                if i + 1 < args.len() {
                    language = args[i + 1].clone();
                    i += 1;
                } else {
                    println!("Error: Missing value for -l/--language");
                    return (message, name, language);
                }
            }
            _ => {
                println!("Error: Invalid argument {}", args[i]);
                return (message, name, language);
            }
        }
        i += 1;
    }

    (message, name, language)
}

pub fn generate_greeting(translations: &HashMap<String, String>, message: &str, name: &str, language: &str) -> String {
    if let Some(greeting) = translations.get(language) {
        if !name.is_empty() {
            format!("{}, {}!", greeting, name)
        } else {
            greeting.to_string()
        }
    } else {
        format!("Error: Unsupported language {}", language)
    }
}

pub fn get_greeting_message(args: &[String]) -> String {
    let translations = load_translations();
    let (message, name, language) = parse_args(args);
    generate_greeting(&translations, &message, &name, &language)
}

fn load_translations() -> HashMap<String, String> {
    let mut file = File::open("src/translations.json").expect("Unable to open translations file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read translations file");
    serde_json::from_str(&data).expect("Unable to parse translations file")
}
