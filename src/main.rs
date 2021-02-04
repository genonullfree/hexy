use ansi_term::Color::RGB;
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


    // Identify if the hex dump flag was used
    if matches.is_present("hexdump") {
        // Read in file to a vec<u8>
        let mut file = File::open(matches.value_of("hexdump").unwrap().to_string()).unwrap();
        let mut a: Vec<u8> = Vec::new();
        let len = file.read_to_end(&mut a).unwrap();

        // Dump colorfully
        colorful_hexdump(&a);

        // Print total length of file
        println!("File length: {} bytes", len);
    } else {
        println!("Unsupported flag");
    }

}

fn printc(a: &u8) {
    print!("{}", RGB((*a << 1) & 0xf0, (*a << 3) & 0xf0, (*a << 5) & 0xf0).paint(format!("{}", *a as char)));
}

fn printx(a: &u8) {
    print!("{}", RGB((*a << 1) & 0xf0, (*a << 3) & 0xf0, (*a << 5) & 0xf0).paint(format!("{:02x} ", a)));
}

fn colorful_hexdump(a: &Vec<u8>) {
    // Iterate through all of the bytes of the file
    for (n,i) in a.into_iter().enumerate() {
        // Every 16 bytes, print summary ascii bytes
        if (n % 16) == 0 {
            // Make sure this is after the first 16 bytes
            if n > 0 {
                // Print ascii bytes in hexdump -C -like style
                printc(&('|' as u8));
                for j in n-16..n {
                    if a[j].is_ascii_graphic() {
                        // Print ascii chars
                        printc(&a[j]);
                    } else {
                        // Print '.' for non-printable chars
                        printc(&('.' as u8));
                    }
                }
                printc(&('|' as u8));
            }
            println!();

            // Print hex address for line
            print!("{:08x} ", n);
        }

        // Add extra space between 8th and 9th hex values
        if (n % 8) == 0 {
            print!(" ");
        }

        // Print hex byte
        printx(i);
    }

    // This is the cleanup for the last line of the hexdump
    if (a.len() % 16) != 0 {
        // Figure out how many bytes are at the end
        let n = a.len() % 16;

        // Add spaces to move cursor
        for _ in 0..(16-n)*3 {
            print!(" ");
        }

        // Add extra space if fewer than 8 chars
        if n <= 8 {
            print!(" ");
        }

        printc(&('|' as u8));
        for i in 0..n {
            if a[a.len()-n+i].is_ascii_graphic() {
                printc(&a[a.len()-n+i]);
            } else {
                printc(&('.' as u8));
            }
        }
        printc(&('|' as u8));
    }

    println!();
}
