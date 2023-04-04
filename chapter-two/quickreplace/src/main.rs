use std::fs;

fn main() {
    let args = parse_arguments();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced = match replace(&args.pattern, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to replace '{}' with '{}' in '{}': {:?}",
                "Error:".red().bold(),
                args.pattern,
                args.replacement,
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct Arguments {
    pattern: String,
    replacement: String,
    filename: String,
    output: String,
}

use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

use std::env;

fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        std::process::exit(1);
    }
    Arguments {
        pattern: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

use regex::Regex;
fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}
