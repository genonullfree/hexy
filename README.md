# hexy

This is a simple hexdump-like utility. It changes the color of the byte based on the ASCII value.

## Usage as a library

```
[dependencies]
hexy = "0.1.1"
```

## API

```
//! This reads in 512 byte chunks and prints as a colorful hexdump
//! input: Must implement the Read trait
pub fn hexyfile<T: std::io::Read>(mut input: T) -> usize

//! This prints the colorful hexdump
//! a: byte slice
//! length: label for left column
//! piece: size of chunk
pub fn hexydump(a: &[u8], length: &usize, piece: &usize)

//! This prints a single u8 as a colorful char, if possible, else a '.'
fn hexy_c(a: &u8)

//! This prints a single u8 as a colorful hex byte {:02x}
fn hexy_x(a: &u8)
```
