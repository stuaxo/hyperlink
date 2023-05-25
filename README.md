# Hyperlink

A Rust CLI tool that generates an OSC-8 hyperlink sequence that points to a file or directory in the file system.


## Usage

```sh
    Usage: hyperlink [OPTIONS] <FILE>
    
    Arguments:
      <FILE>  The file or directory to generate a hyperlink for
    
    Options:
      -t, --text <TEXT>                Set custom text for the hyperlink
      -r, --relative-to <RELATIVE_TO>  Make the file path relative to the specified folder
      -s, --shorten                    Shorten the file path by using tilde instead of the home directory
      -h, --help                       Print help
      -V, --version                    Print version
```

## Developers

See `DEVELOPERS.md` for more information on how to build and install the application.
