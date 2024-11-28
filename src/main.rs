use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn load_translations() -> HashMap<String, String> {
    let mut file = File::open("src/translations.json").expect("Unable to open translations file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read translations file");
    serde_json::from_str(&data).expect("Unable to parse translations file")
}

fn print_usage() {
    println!("Usage: hello [-m MESSAGE] [-n NAME] [-l LANGUAGE]");
    println!("Options:");
    println!("  -m, --message   Custom greeting message");
    println!("  -n, --name      Name for personalized greeting");
    println!("  -l, --language  Language for the greeting message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let translations = load_translations();

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
                    print_usage();
                    return;
                }
            }
            "-n" | "--name" => {
                if i + 1 < args.len() {
                    name = args[i + 1].clone();
                    i += 1;
                } else {
                    println!("Error: Missing value for -n/--name");
                    print_usage();
                    return;
                }
            }
            "-l" | "--language" => {
                if i + 1 < args.len() {
                    language = args[i + 1].clone();
                    i += 1;
                } else {
                    println!("Error: Missing value for -l/--language");
                    print_usage();
                    return;
                }
            }
            _ => {
                println!("Error: Invalid argument {}", args[i]);
                print_usage();
                return;
            }
        }
        i += 1;
    }

    if let Some(greeting) = translations.get(&language) {
        if !name.is_empty() {
            println!("{}, {}!", greeting, name);
        } else {
            println!("{}", greeting);
        }
    } else {
        println!("Error: Unsupported language {}", language);
        print_usage();
    }
}
