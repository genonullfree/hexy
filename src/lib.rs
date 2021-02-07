use ansi_term::Color::RGB;

pub fn hexyfile<T: std::io::Read>(mut input: T) -> usize {
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
        hexydump(&a, &len, &chunk);
        len += chunk;
    }
    println!();

    // Return total length
    len
}

pub fn hexy_c(a: &u8) {
    let mut c = *a as char;
    if !a.is_ascii_graphic() {
        c = '.';
    }

    print!(
        "{}",
        RGB((*a << 1) & 0xf0, (*a << 3) & 0xf0, (*a << 5) & 0xf0).paint(format!("{}", c))
    );
}

pub fn hexy_x(a: &u8) {
    print!(
        "{}",
        RGB((*a << 1) & 0xf0, (*a << 3) & 0xf0, (*a << 5) & 0xf0).paint(format!("{:02x}", a))
    );
}

pub fn hexydump(a: &[u8], length: &usize, piece: &usize) {
    let mut len = *length;
    let chunk = *piece;
    // Iterate through all of the bytes of the file
    for (n, i) in a.into_iter().enumerate() {
        if n == chunk {
            break;
        }
        // Every 16 bytes, print summary ascii bytes
        if (n % 16) == 0 {
            // Make sure this is after the first 16 bytes
            if n > 0 {
                // Print ascii bytes in hexdump -C -like style
                hexy_c(&('|' as u8));
                for j in n - 16..n {
                    if a[j].is_ascii_graphic() {
                        // Print ascii chars
                        hexy_c(&a[j]);
                    } else {
                        // Print '.' for non-printable chars
                        hexy_c(&('.' as u8));
                    }
                }
                hexy_c(&('|' as u8));
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
        hexy_x(i);
        print!(" ");
    }

    let mut n = 0;
    // This is the cleanup for the last line of the hexdump
    if (chunk % 16) != 0 {
        // Figure out how many bytes are at the end
        n = chunk % 16;

        // Add spaces to move cursor
        for _ in 0..(16 - n) * 3 {
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
    hexy_c(&('|' as u8));
    for i in 0..n {
        if a[chunk - n + i].is_ascii_graphic() {
            hexy_c(&a[chunk - n + i]);
        } else {
            hexy_c(&('.' as u8));
        }
    }
    hexy_c(&('|' as u8));
}
