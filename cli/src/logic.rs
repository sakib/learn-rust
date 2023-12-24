use regex::Regex;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("Usage: <target-string> <replacement-string> <input-file> <output-file>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments given. Expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, rep).to_string())
}

fn read_write(args: &Arguments) {
    let data = match std::fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} failed to read from file {}: {:?}",
                "Error".red().bold(),
                args.input_file,
                e
            );
            std::process::exit(2);
        }
    };

    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(3);
        }
    };

    match std::fs::write(&args.output_file, &replace_data) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to read from file {}: {:?}",
                "Error".red().bold(),
                args.input_file,
                e
            );
            std::process::exit(4);
        }
    };
}

pub fn run() {
    let args = parse_args();
    read_write(&args);
}
