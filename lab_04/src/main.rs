use std::{fs, io};

// P1
fn actual_len_and_bytes(file: &str) -> (&str, usize, &str, usize) {
    // Keep track of the maximum bytes and the matching line
    let mut max_bytes = 0;
    let mut line_max_bytes = "";

    // Keep track of the maximum characters and the matching line
    let mut max_chars = 0;
    let mut line_max_chars = "";

    for line in file.lines() {
        // Get line length in bytes
        let nr_bytes = line.len();
        if nr_bytes > max_bytes {
            max_bytes = nr_bytes;
            line_max_bytes = line;
        }

        // Get actual character count
        let nr_chars = line.chars().count();
        if nr_chars > max_chars {
            max_chars = nr_chars;
            line_max_chars = line;
        }
    }

    (line_max_bytes, max_bytes, line_max_chars, max_chars)
}

// P2
fn rot13(text: &str) -> Result<String, String> {
    let mut rezultat = String::new();

    for c in text.chars() {
        // If a not ASCII character is found we return an error
        if !c.is_ascii() {
            return Err(format!("Erorr: Non-ASCII  '{}' not supported!", c));
        }

        let mut b = c as u8;

        // The algorithm for R13
        if c.is_ascii_lowercase() {
            b = (b - b'a' + 13) % 26 + b'a';
        } else if c.is_ascii_uppercase() {
            b = (b - b'A' + 13) % 26 + b'A';
        }

        rezultat.push(b as char);
    }

    Ok(rezultat)
}

// P3
fn expand_abbreviations(phrase: &str) -> String {
    let mut result = String::new();

    for word in phrase.split_whitespace() {
        // Match abbreviation to its expanded form
        let expanded = match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word, // keep the same word if it's not an abbreviation
        };

        // Add a space before adding the next word
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(expanded);
    }

    result
}

// P4
fn print_hosts(content: &str) {
    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comment lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Split line by whitespace into words
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        // Take only the first two columns (IP and Host)
        if parts.len() >= 2 {
            let ip = parts[0];
            let host = parts[1];

            println!("{} => {}", host, ip);
        }
    }
}

fn main() -> io::Result<()> {
    // P1
    let file = fs::read_to_string("src/text_input.txt")?;

    let (l_bytes, b, l_chars, c) = actual_len_and_bytes(&file);

    println!("Longest line (bytes): '{}' ({} bytes)", l_bytes, b);
    println!("Longest line (characters): '{}' ({} caractere)", l_chars, c);

    // P2

    let file1 = fs::read_to_string("src/text_input_P2.txt")?;

    match rot13(&file1) {
        Ok(encodded) => {
            println!("Encodded text ROT13:\n{}", encodded);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    // P3
    let content = fs::read_to_string("src/text_input_P3.txt")?;

    let result = expand_abbreviations(&content);

    println!("Original: {}", content.trim());
    println!("Expanded: {}", result);

    // P4
    let hosts_path = r"C:\Windows\System32\drivers\etc\hosts";
    let hosts_content = match fs::read_to_string(hosts_path) {
        Ok(text) => text,
        Err(_) => fs::read_to_string("src/hosts.txt")?,
    };

    print_hosts(&hosts_content);

    let hosts_content = fs::read_to_string("src/hosts.txt")?;
    print_hosts(&hosts_content);

    Ok(())
}
