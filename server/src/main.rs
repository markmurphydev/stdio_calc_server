use std::io;

fn main() {
    // Read stdin in a loop.
    // Whenever we read a line, print it plus an exclamation mark.
    let stdin = io::stdin();
    let mut buf = String::new();
    loop {
        buf.truncate(0);
        let bytes_read = stdin.read_line(&mut buf).unwrap();
        if bytes_read == 0 {
            return;
        }

        // Trim off the newline.
        let request = buf.trim_end();
        println!("{request}!");
    }
}
