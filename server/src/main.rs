use std::io;

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
    let mut buf = String::new();
    loop {
        buf.truncate(0);
        let bytes_read = stdin.read_line(&mut buf).unwrap();
        if bytes_read == 0 {
            return Ok(());
        }

        let expr: Expr = serde_json::from_str(&buf)?;
        eprintln!("{expr:?}");
        let res = eval(expr);
        println!("{res}");
    }
}
