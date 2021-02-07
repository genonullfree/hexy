# hexy

This is a simple hexdump-like utility. It changes the color of the byte based on the ASCII value.

## Usage as a library

```
[dependencies]
hexy = "0.1.0"
```

## API

```
pub fn hexyfile<T: std::io::Read>(mut input: T) -> usize

pub fn hexydump(a: &[u8], length: &usize, piece: &usize)
```
