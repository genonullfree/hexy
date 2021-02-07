use clap::{App, Arg};
use hexy::*;
use std::fs::File;
use std::io;

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

    // Identify if the hex dump flag was used
    if matches.is_present("hexdump") {
        // Read in file to a vec<u8>
        let file = File::open(matches.value_of("hexdump").unwrap().to_string()).unwrap();

        // Read file
        let len = hexyfile(file);

        // Print footer info
        println!("File length: {} bytes", len);
    } else {
        let standardin = io::stdin();
        let file = standardin.lock();

        // Read stdin
        let len = hexyfile(file);

        // Print footer info
        println!("File length: {} bytes", len);
    }
}
