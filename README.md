# Hello!

## Description

The `hello` program is a simple command-line tool that outputs a greeting message. It supports customizable greetings, personalized messages, and multilingual support.

## Usage

```
hello [-m MESSAGE] [-n NAME] [-l LANGUAGE]
```

### Options

- `-m`, `--message`: Custom greeting message
- `-n`, `--name`: Name for personalized greeting
- `-l`, `--language`: Language for the greeting message

### Examples

1. Basic greeting:
   ```
   hello
   ```
   Output: `Hello, World!`

2. Custom message:
   ```
   hello -m "Greetings"
   ```
   Output: `Greetings`

3. Personalized greeting:
   ```
   hello -n Alice
   ```
   Output: `Hello, Alice!`

4. Multilingual greeting:
   ```
   hello -l ja
   ```
   Output: `こんにちは、世界！`

5. Custom message with name:
   ```
   hello -m "Hi" -n Bob
   ```
   Output: `Hi, Bob!`

### Error Handling

When invalid command-line arguments are provided, the program will display an error message indicating the issue and provide usage information to guide the user on how to use the program correctly. The program will then exit with a non-zero status code to indicate an error.

If invalid command-line arguments are provided, the program will default to the basic functionality of displaying a simple greeting message, such as "Hello, World!". Optionally, a warning message may be displayed indicating that the provided arguments were invalid and the program is defaulting to the basic functionality.
