use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    process::{Command, Stdio},
};

fn main() {
    // Get "server" executable, which should be in the same directory.
    let mut server_path = env::current_exe().unwrap();
    server_path.set_file_name("server");

    let mut server_proc = Command::new(server_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut server_stdin = server_proc.stdin.take().unwrap();
    let server_stdout = server_proc.stdout.take().unwrap();
    let mut server_stdout = BufReader::new(server_stdout);

    // In a loop, read a line from our stdin,
    // send it to the server,
    // then print the response.
    let mut stdin = io::stdin().lock();
    let mut user_input = String::new();
    let mut server_response = String::new();
    loop {
        user_input.truncate(0);
        server_response.truncate(0);
        let bytes_read = stdin.read_line(&mut user_input).unwrap();
        if bytes_read == 0 {
            return;
        }

        // Send input as request to server
        server_stdin.write_all(user_input.as_bytes()).unwrap();
        server_stdin.flush().unwrap();

        // Wait on response from server
        let bytes_read = server_stdout.read_line(&mut server_response).unwrap();
        if bytes_read == 0 {
            panic!("Server should always respond.");
        }

        let server_response = server_response.trim_end();
        println!("{server_response}");
    }
}
