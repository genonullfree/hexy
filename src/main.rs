use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("hexy")
        .version("0.1.0")
        .author("geno")
        .about("colorful hex dump output")
        .arg(
            Arg::with_name("hexdump")
                .short("f")
                .long("hexdump")
                .help("file to dump hexily")
                .takes_value(true),
        )
        .get_matches();


    if matches.is_present("hexdump") {
        let mut file = File::open(matches.value_of("hexdump").unwrap().to_string()).unwrap();
        let mut a = Vec::new();
        let len = file.read_to_end(&mut a).unwrap();
        println!("File length: {} bytes", len);
    } else {
        println!("Unsupported flag");
    }


}
