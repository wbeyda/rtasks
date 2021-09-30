use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, White} ;
//use ansi_term::Colour;
//use ansi_term::Style;



fn main() {
    println!("Demonstrating {} and {}!",
            Blue.bold().paint("blue bold"),
            Yellow.underline().paint("yellow underline"));

    println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
    //let rb = Colour.Red.blink();
    //println!("{}", Style::new().bold().paint("bold"));
    //println!("This is in red: {}", rb.paint("a red string") );
    /*
    println!("This is in black: {}", Black.paint("a red string"));
    println!("This is in red: {}", rb.paint("a red string") );
    println!("This is in green: {}", Green.paint("a red string"));
    println!("This is in yellow: {}", Yellow.paint("a red string"));
    println!("This is in blue: {}", Blue.paint("a red string"));
    println!("This is in purple: {}", Purple.paint("a red string"));
    println!("This is in cyan: {}", Cyan.paint("a red string"));
    println!("This is in white: {}", White.paint("a red string"));
    */
}
