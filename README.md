# hexy

This is a simple utility that can print a byte as an ASCII char or hex value. It will change the color of the byte based on the byte value.  
This can be used with some builtin functions to provide a hexdump-like interface, or for byte-by-byte hex value or ASCII character printing.

## Usage as a library

```
[dependencies]
hexy = "0.1.2"
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
pub fn hexy_c(a: &u8)

//! This prints a single u8 as a colorful hex byte {:02x}
pub fn hexy_x(a: &u8)
```
