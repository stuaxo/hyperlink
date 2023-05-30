# Hyperlink

A Rust CLI tool that generates an OSC-8 hyperlink sequence that points to a file or directory in the file system.

## Usage

```sh
    Usage: hyperlink [OPTIONS] <FILE>
    
    Arguments:
      <FILE>  The file or directory to generate a hyperlink for
    
    Options:      
      -t, --text <TEXT>                Custom text for the hyperlink
      -r, --relative-to <RELATIVE_TO>  If not using custom text, file path is relative to specified folder
      -s, --shorten                    If not using custom text, shorten the file path by using tilde
          --osc8                       Always output osc8 hyperlinks even if stdout is not a terminal
      -h, --help                       Print help
      -V, --version                    Print version
```

## Developers

See `DEVELOPERS.md` for more information on how to build and install the application.
