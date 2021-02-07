# hexy

This is a simple hexdump-like utility. It changes the color of the byte based on the ASCII value. This was mostly for fun but could also come in handy in some situations I guess maybe.

## Usage as an executable
```
USAGE:
    hexy [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --hexdump <hexdump>    file to dump hexily
```

## Executable examples

Pass a file to hexy:
```
./hexy -f <filename>
```

Pipe input to hexy:
```
cat <filename> | ./hexy
```

## Usage as a library

```
[dependencies]
hexy = "0.1.0"
```
