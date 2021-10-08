use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, White} ;
//use ansi_term::Colour;
//use ansi_term::Style;
extern crate clap;
use clap::{Arg, App, SubCommand};
mod lib;


pub fn main() {
    lib::check_for_database();
    lib::touch_database_file_or_throw_an_error();

    let matches = App::new("rtasks")
        .version("1.0")
        .author("<github.com/wbeyda>")
        .about("Set's tasks")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::with_name("debug")
            .short("d")
            .help("print debug information verbosely")))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("{}{}", Green.bold().paint("Value for config: "), config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("{}{}", Blue.bold().paint("Using input file: "), matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("{}{}!", Blue.bold().paint("Printing debug "), Yellow.underline().paint("info ..."));
        } else {
            println!("Printing normally...");
            println!("{}", Yellow.underline().paint("Printing normal info ..."));
        }
    }

//     yup I tested all the colors and styles. 
//     println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
//     let rb = Colour.Red.blink();
//     println!("{}", Style::new().bold().paint("bold"));
//     println!("This is in red: {}", rb.paint("a red string"));
//     println!("This is in black: {}", Black.paint("a red string"));
//     println!("This is in red: {}", rb.paint("a red string") );
//     println!("This is in green: {}", Green.paint("a red string"));
//     println!("This is in yellow: {}", Yellow.paint("a red string"));
//     println!("This is in blue: {}", Blue.paint("a red string"));
//     println!("This is in purple: {}", Purple.paint("a red string"));
//     println!("This is in cyan: {}", Cyan.paint("a red string"));
//     println!("This is in white: {}", White.paint("a red string"));
}
