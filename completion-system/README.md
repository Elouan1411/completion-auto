# A Rust based Key stroke recorder

A key logger based on Rust. It tworks on Debian based operating systems only. It works by listening on `/dev/inputs/events*` and prints the key strokes to the terminal.

## Usage

1. Clone the repository
2. Run `cargo build`
3. Run `sudo ./target/debug/keylogger`
4. Press any key to see the key strokes
5. Press `Ctrl + C` to exit
6. Run `sudo ./target/debug/keylogger > keylog.txt` to save the key strokes to a file

## Todo
* Convert keycodes to key characters
* Send log to a server
* Make it work on other operating systems
 