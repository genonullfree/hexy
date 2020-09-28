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
        let mut a: Vec<u8> = Vec::new();
        let len = file.read_to_end(&mut a).unwrap();

        colorful_hexdump(&a);

        println!("File length: {} bytes", len);
    } else {
        println!("Unsupported flag");
    }

}

fn colorful_hexdump(a: &Vec<u8>) {

    for (n,i) in a.into_iter().enumerate() {
        if (n % 16) == 0 {
            if n > 0 {
                print!("|");
                for j in n-16..n {
                    if a[j].is_ascii_graphic() {
                        print!("{}", a[j] as char);
                    } else {
                        print!(".");
                    }
                }
                print!("|");
            }
            println!();
            print!("{:08x} ", n);
        }

        if (n % 8) == 0 {
            print!(" ");
        }

        print!("{:02x} ", i);
    }

    if (a.len() % 16) != 0 {
        let mut n = a.len() % 16;

        for _ in 0..(16-n)*3 {
            print!(" ");
        }
        if n <= 8 {
            n += 1;
        }

        print!("|");
        for i in 0..n {
            if a[a.len()-n+i].is_ascii_graphic() {
                print!("{}", a[a.len()-n+i] as char);
            } else {
                print!(".");
            }
        }
        print!("|");
    }

    println!();
}
