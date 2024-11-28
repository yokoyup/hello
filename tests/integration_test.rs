#[cfg(test)]
mod tests {
    use hello::{parse_args, generate_greeting, get_greeting_message};
    use std::collections::HashMap;

    #[test]
    fn test_parse_args() {
        let args = vec![
            "hello".to_string(),
            "-m".to_string(),
            "Greetings".to_string(),
            "-n".to_string(),
            "Alice".to_string(),
            "-l".to_string(),
            "ja".to_string(),
        ];
        let (message, name, language) = parse_args(&args);
        assert_eq!(message, "Greetings");
        assert_eq!(name, "Alice");
        assert_eq!(language, "ja");
    }

    #[test]
    fn test_generate_greeting() {
        let mut translations = HashMap::new();
        translations.insert("en".to_string(), "Hello, World!".to_string());
        translations.insert("ja".to_string(), "こんにちは、世界！".to_string());

        let greeting = generate_greeting(&translations, "Hello, World!", "", "en");
        assert_eq!(greeting, "Hello, World!");

        let greeting = generate_greeting(&translations, "こんにちは、世界！", "Alice", "ja");
        assert_eq!(greeting, "こんにちは、世界！, Alice!");
    }

    #[test]
    fn test_get_greeting_message() {
        let args = vec![
            "hello".to_string(),
            "-m".to_string(),
            "Greetings".to_string(),
            "-n".to_string(),
            "Alice".to_string(),
            "-l".to_string(),
            "ja".to_string(),
        ];
        let greeting = get_greeting_message(&args);
        assert_eq!(greeting, "こんにちは、世界！, Alice!");
    }

    #[test]
    fn test_invalid_argument() {
        let args = vec![
            "hello".to_string(),
            "-x".to_string(),
        ];
        let greeting = get_greeting_message(&args);
        assert_eq!(greeting, "Hello, World!");
    }

    #[test]
    fn test_missing_value_for_message() {
        let args = vec![
            "hello".to_string(),
            "-m".to_string(),
        ];
        let greeting = get_greeting_message(&args);
        assert_eq!(greeting, "Hello, World!");
    }

    #[test]
    fn test_missing_value_for_name() {
        let args = vec![
            "hello".to_string(),
            "-n".to_string(),
        ];
        let greeting = get_greeting_message(&args);
        assert_eq!(greeting, "Hello, World!");
    }

    #[test]
    fn test_missing_value_for_language() {
        let args = vec![
            "hello".to_string(),
            "-l".to_string(),
        ];
        let greeting = get_greeting_message(&args);
        assert_eq!(greeting, "Hello, World!");
    }

    #[test]
    fn test_unsupported_language() {
        let args = vec![
            "hello".to_string(),
            "-l".to_string(),
            "xx".to_string(),
        ];
        let greeting = get_greeting_message(&args);
        assert_eq!(greeting, "Error: Unsupported language xx");
    }
}
