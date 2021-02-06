use ansi_term::Color::RGB;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Read};

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
        let len = read_file(file);

        // Print footer info
        println!("File length: {} bytes", len);

    } else {
        let standardin = io::stdin();
        let mut file = standardin.lock();

        // Read stdin
        let len = read_file(file);

        // Print footer info
        println!("File length: {} bytes", len);
    }
}

fn read_file<T: std::io::Read>(mut input: T) -> usize {
    let mut len: usize = 0;
    loop {
        // Read in up to 512 bytes at a time
        let mut a: [u8; 512] = [0; 512];
        let chunk = input.read(&mut a).unwrap();

        // If read was empty we're done
        if chunk == 0 {
            break;
        }

        // Print this chunk hexily
        colorful_hexdump(&a, &len, &chunk);
        len += chunk;
    }
    println!();

    // Return total length
    len
}

fn printc(a: &u8) {
    print!("{}", RGB((*a << 1) & 0xf0, (*a << 3) & 0xf0, (*a << 5) & 0xf0).paint(format!("{}", *a as char)));
}

fn printx(a: &u8) {
    print!("{}", RGB((*a << 1) & 0xf0, (*a << 3) & 0xf0, (*a << 5) & 0xf0).paint(format!("{:02x} ", a)));
}

fn colorful_hexdump(a: &[u8], length: &usize, piece: &usize) {
    let mut len = *length;
    let chunk = *piece;
    // Iterate through all of the bytes of the file
    for (n,i) in a.into_iter().enumerate() {
        if n == chunk {
            break;
        }
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
            print!("{:08x} ", len);
            len += 16;
        }

        // Add extra space between 8th and 9th hex values
        if (n % 8) == 0 {
            print!(" ");
        }

        // Print hex byte
        printx(i);
    }

    let mut n = 0;
    // This is the cleanup for the last line of the hexdump
    if (chunk % 16) != 0 {
        // Figure out how many bytes are at the end
        n = chunk % 16;

        // Add spaces to move cursor
        for _ in 0..(16-n)*3 {
            print!(" ");
        }

        // Add extra space if fewer than 8 chars
        if n <= 8 {
            print!(" ");
        }
    }

    if n == 0 {
        n = 16;
    }
    printc(&('|' as u8));
    for i in 0..n {
        if a[chunk-n+i].is_ascii_graphic() {
            printc(&a[chunk-n+i]);
        } else {
            printc(&('.' as u8));
        }
    }
    printc(&('|' as u8));
}
