use std::io::{self, BufRead, BufReader};

use shared::Expr;

fn eval(expr: Expr) -> i64 {
    match expr {
        Expr::Int(int) => int,
        Expr::Add(lhs, rhs) => eval(*lhs) + eval(*rhs),
        Expr::Sub(lhs, rhs) => eval(*lhs) - eval(*rhs),
        Expr::Mul(lhs, rhs) => eval(*lhs) * eval(*rhs),
        Expr::Div(lhs, rhs) => eval(*lhs) / eval(*rhs),
    }
}

fn main() -> anyhow::Result<()> {
    // Read stdin in a loop.
    // Whenever we read a line, print it plus an exclamation mark.
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin);
    let mut buf = Vec::new();
    loop {
        buf.truncate(0);
        let bytes_read = stdin.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let expr: Expr = postcard::from_bytes(&buf)?;
        eprintln!("{expr:?}");
        let res = eval(expr);
        println!("{res}");
    }
}
