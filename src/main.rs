extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select};
use std::io;
use std::io::prelude::*;
use std::process;
use std::time::Instant;

fn exit_pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to exit...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn nice_exit() {
    println!("");
    println!("Thanks for using this software.");
    exit_pause();
    process::exit(0);
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

fn main() {
    let file_format_options = &[
        "Comma separated (,)",
        "Semicolon separated (;)",
        "Tab separated",
    ];

    let decimal_format_options = &["Period (.)", "Comma (,)"];

    greeting();
    let file_format_input_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format is your file in ?")
        .items(&file_format_options[..])
        .default(0)
        .interact_opt()
        .unwrap();
    if !file_format_input_selection.is_some() {
        nice_exit()
    }

    let decimal_format_input_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format are your decimals in ?")
        .items(&decimal_format_options[..])
        .default(0)
        .interact_opt()
        .unwrap();
    if !decimal_format_input_selection.is_some() {
        nice_exit()
    }

    let file_format_output_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format do you want the decimals to output in ?")
        .items(&file_format_options[..])
        .default(1)
        .interact_opt()
        .unwrap();
    if !file_format_output_selection.is_some() {
        nice_exit()
    }

    let decimal_format_output_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which format do you want the decimals to output in ?")
        .items(&decimal_format_options[..])
        .default(1)
        .interact_opt()
        .unwrap();
    if !decimal_format_output_selection.is_some() {
        nice_exit()
    }

    let now = Instant::now();
    println!("Duration of the conversion: {}", now.elapsed().as_secs());

    nice_exit();
}
