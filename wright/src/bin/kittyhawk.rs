extern crate wright;
//extern crate clap;

//use clap::{App, Arg, AppSettings};
use wright::version::VERSION;
use std::process::exit;

fn main() {
    println!("Wright's jvm implementation is not yet implemented as of version {}.", VERSION);
    exit(1);
    /*let matches = App::new("Kittyhawk")
        .version(VERSION)
        .setting(AppSettings::ColorAlways)
        .author("Antonia Calia-Bogan (github.com/Alfriadox)")
        .about("A JVM interpreter implemented in rust as part of the Wright Programming Language.")
        .arg(Arg::with_name("FILE")
            .takes_value(true)
            .required(true)
            .help("Class file input.")
        )
        .arg(Arg::with_name("DUMP")
            .short("d")
            .long("dump")
            .help("Prints the file's bytecodes.")
        )
        .get_matches();*/
}