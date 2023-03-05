use crate::formatting::current_timestamp;
use std::io::{stdout, Write};

pub fn render(task_name: &str) {
    println!("> BIG_IRON @ {:?}: {:?}", current_timestamp(), task_name);
    stdout().flush().unwrap(); // flush the output to ensure it's displayed immediately
}

fn delete_previous_lines(n: u16) {
    let mut stdout = stdout();
    write!(stdout, "\x1b[{}F\x1b[0J", n).unwrap();
    stdout.flush().unwrap();
}
