use std::path::Path;
use std::env;
use std::fs::OpenOptions;

///print the working directory 
fn print_working_directory() -> std::io::Result<()> {
     let path = env::current_dir()?;
     println!("The working directory is {}", path.display());
     Ok(())
}


///check for sqlite3 database
pub fn check_for_database() {

    let dbcheck: bool = Path::new("db.sqlite3").exists();

    println!("dbcheck = {}", dbcheck);

    if dbcheck == true {
        println!("DATABASE FOUND");
        let pwd = print_working_directory();
        pwd;
        //try to check why the path doesn't exist either it doesn't exist or current permissions
    } else {
        println!("No DATABASE FOUND");
        let pwd = print_working_directory();
        pwd;

    }
}


/// creates an sqlite3 database file or throws an error
pub fn touch_database_file_or_throw_an_error() {
    let file = OpenOptions::new().write(true).create(true).open("db.sqlite3");
    file;
}

