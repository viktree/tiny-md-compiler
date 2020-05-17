use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

// --- Usage ----------------------------------------------------------------------------

fn print_short_banner() {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    println!("{}", the_title);
}

fn print_long_banner() {
    let names = env!("CARGO_PKG_AUTHORS");
    let homepage = env!("CARGO_PKG_HOMEPAGE");
    print_short_banner();
    println!("Written by: {}\nHomepage: {}", names, homepage);
    println!("Usage: tinymd <somefile>.md\n");
}

fn usage() {
    print_long_banner();
}

// --- Parser ---------------------------------------------------------------------------

fn write_tokens_to_file(filename: &str, tokens : Vec<String>) {
    // Create an output file based on the input file, minus ".md"
    let mut outputfilename = String::from(&filename[..filename.len() - 3]);
    outputfilename.push_str(".html");

    let mut outfile =
        File::create(outputfilename.to_string()).expect("[ ERROR ] Could not create output file!");

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] HTML file emitted!");
}

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let inputfilename = Path::new(filename);
    let file = File::open(&inputfilename).expect("[ERROR] Failed to open file!");

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut output_line = String::new();
        // Instead of writing a matcher to handle error checking each time,
        // we can do a Rust trick called unwrapping.
        //
        // Verbose way:
        //
        // let line_contents = match line {
        //       Ok(contents) => contents,
        //       Err(e) => panic!("Garbage: {}", e.description())
        // };
        //
        // When you unwrap a result object, you are telling Rust that you
        //   1) expect the value to be available
        //   2) don’t care if the line is garbage
        let line_contents = line.unwrap().to_string();

        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n")
                }
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }
                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            }
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_contents);
            }
        }

        // Close any dangling tags
        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }
        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        // Avoid pushing blank lines
        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    println!("[ INFO ] Parsing complete!");

    write_tokens_to_file(filename, tokens);
}

// --- Main -----------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => usage(),
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done fucked up!)");
            usage();
        }
    }
}

// --------------------------------------------------------------------------------------
