# Developer Guide

## Prerequisites

- Rust programming language (install using [rustup](https://rustup.rs/))

## Installation

1. Install Rust using rustup:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   Follow the on-screen instructions to complete the installation.

2. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/hyperlink.git
    cd hyperlink
    ```

   Replace yourusername with your GitHub username or the appropriate path to the repository.

3. Build the project:

   ```sh
   cargo build --release
   ```

4. Install the application:

   ```sh
   cargo install --path .
   ```

   This will install the hyperlink binary to the $HOME/.cargo/bin directory. Ensure this directory is in your PATH to
   use the hyperlink command.

# Usage

Run the hyperlink command with the required arguments and options:

   ```sh
   $ hyperlink PATH [OPTIONS] [TEXT]
   ````

For more information on available options, run:

   ```sh
   $ hyperlink --help
   ```
