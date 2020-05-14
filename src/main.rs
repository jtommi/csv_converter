extern crate csv;
extern crate dialoguer;
extern crate regex;
extern crate ropey;

use ropey::Rope;
use csv::{ReaderBuilder, WriterBuilder};
use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*,BufReader, BufWriter};
use std::time::Instant;

fn run() -> Result<(), Box<dyn Error>> {
    let input_file_path = get_arg(1)?;
    let output_file_path = get_arg(2)?;
    let input_file = File::open(&input_file_path)?;

    let file_format_options = &[
        "Comma separated (,)",
        "Semicolon separated (;)",
        "Tab separated",
    ];

    let decimal_format_options = &["Period (.)", "Comma (,)"];

    greeting();
    let file_format_input_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format is your file in ?")
        .default(0)
        .items(&file_format_options[..])
        .interact_opt()
        .unwrap();

    let mut input_column_delimiter: u8 = b' ';

    match file_format_input_selection {
        Some(0) => input_column_delimiter = b',',
        Some(1) => input_column_delimiter = b';',
        Some(2) => input_column_delimiter = b'\t',
        _ => {
            nice_exit()?;
        },
    }

    let re_period = Regex::new(r"^(\d+)\.(\d+)$").unwrap();
    let re_comma = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let re;

    let decimal_format_input_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format are your decimals in ?")
        .items(&decimal_format_options[..])
        .default(0)
        .interact_opt()
        .unwrap();
    match decimal_format_input_selection {
        Some(0) => re = re_period,
        Some(1) => re = re_comma,
        _ => {
            re = Regex::new(r"").unwrap();
            nice_exit()?;
        }
    }

    let file_format_output_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format do you want the output file to be in ?")
        .items(&file_format_options[..])
        .default(1)
        .interact_opt()
        .unwrap();
    let mut output_column_delimiter: u8 = b' ';
    match file_format_output_selection {
        Some(0) => output_column_delimiter = b',',
        Some(1) => output_column_delimiter = b';',
        Some(2) => output_column_delimiter = b'\t',
        _ => {
            nice_exit()?;
        },
    }

    let replace_with;
    let decimal_format_output_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format do you want the decimals to output in ?")
        .items(&decimal_format_options[..])
        .default(1)
        .interact_opt()
        .unwrap();
    match decimal_format_output_selection {
        Some(0) => replace_with = ".",
        Some(1) => replace_with = ",",
        _ => {
            replace_with = "";
            nice_exit()?;
        }
    }

    let now = Instant::now();

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(output_file_path)
        .unwrap();

    let text = Rope::from_reader(
        BufReader::new(File::open(&input_file_path)?)
    )?;
    
    let mut lines = text.lines();
    while let Some(s) = lines.next(){
        print!("{}",s);
    };
    
    /* text.write_to(
        BufWriter::new(File::create(&output_file_path)?)
    )?; */

    println!(
        "Duration of the conversion: {} seconds",
        now.elapsed().as_millis()/1000
    );

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
    Some(nice_exit());
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_arg(number: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(number) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn exit_pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to exit...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn nice_exit() -> Result<(), Box<dyn Error>> {
    println!("");
    println!("Thanks for using this software.");
    exit_pause();
    Ok(())
}

fn greeting() {
    println!("=========================================================");
    println!("|  Welcome to the CSV converter utility                 |");
    println!("|                                                       |");
    println!("|  This utility let's you change the column separator,  |");
    println!("|  as well as the decimal separator.                    |");
    println!("|                                                       |");
    println!("|  You can exit the menu using ESC or q                 |");
    println!("|  The conversion process can be aborted using CTRL-C   |");
    println!("=========================================================");
    println!("");
}
