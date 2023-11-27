
# Cyberdevel's Desktop Application

## Overview

This is the desktop application for Cyberdevel written in Rust.

## Requirements

- Windows 7 or later, Linux or macOS 10.15 or later
- Rust programming language environment (for compiling the application, not needed for running it from pre-compiled binaries)

## Installation and Building

### Running the application

- Go to the [releases page](https://github.com/Azx5G/Cyberdevel-rs/releases) and download the latest release for your operating system to run the app.

### Building from source

### Installing Rust

To compile this project, you need to have Rust and Cargo installed on your system. If you don't have them installed, follow these steps:

#### Using the script

- Requirements : 
1. Python >=3 installed on your system
2. Internet Access
3. Administrator Access

Simply execute the installrust.py script with
   ```
python3 installrust.py
   ```
or
   ```
py -3 installrust.py
   ```
#### Manual Installation

#### Windows

1. Download and run the Rust installer from [the official Rust website](https://www.rust-lang.org/tools/install).
2. Follow the on-screen instructions to complete the installation.

#### Linux and macOS

1. Open a terminal.
2. Run the following command:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Follow the on-screen instructions to complete the installation.

### Compiling the Application

Once Rust and Cargo are installed:

1. Open a terminal (or command prompt on Windows).
2. Navigate to the directory where you have cloned or downloaded this project.
3. Run the following command to compile the application:
   ```
   cargo build --release
   ```
4. The compiled binary will be located in `target/release`.

## Usage

This is only a test application. It does not have any real functionality.

## License

- **[MIT license](http://opensource.org/licenses/mit-license.php)**

## Contact

- **Organization:** Cyberdevel
- **Maintainer:** Azx5G
- **Website:** https://cyberdevel.com
