mod parse;

use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use parse::Parser;

fn main() -> anyhow::Result<()> {
    // Get "server" executable, which should be in the same directory.
    let mut server_path = env::current_exe()?;
    server_path.set_file_name("server");

    let mut server_proc = Command::new(server_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut server_stdin = server_proc
        .stdin
        .take()
        .ok_or(anyhow::Error::msg("No server stdin"))?;
    let server_stdout = server_proc
        .stdout
        .take()
        .ok_or(anyhow::Error::msg("No server stdout"))?;
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
        let bytes_read = stdin.read_line(&mut user_input)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let Ok(parser) = Parser::new(&user_input) else {
            continue;
        };
        let Ok(Some(ast)) = parser.parse() else {
            continue;
        };

        let ast_string = serde_json::to_string(&ast)?;

        // Send input as request to server
        server_stdin.write_all(format!("{ast_string}\n").as_bytes())?;
        server_stdin.flush()?;

        // Wait on response from server
        let bytes_read = server_stdout.read_line(&mut server_response)?;
        if bytes_read == 0 {
            panic!("Server should always respond.");
        }

        let server_response = server_response.trim_end();
        println!("{server_response}");
    }
}
