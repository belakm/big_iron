use std::io::{stdout, Write};

pub fn render(current_time: &String, task_name: &String, task_time: &String) {
    delete_previous_lines(3);
    print!(
        "=================
BIG - IRON @ {:?}
Last task: {:?} at {:?}
=================",
        current_time, task_name, task_time
    );
    stdout().flush().unwrap(); // flush the output to ensure it's displayed immediately
}

fn delete_previous_lines(n: u16) {
    let mut stdout = stdout();
    write!(stdout, "\x1b[{}F\x1b[0J", n).unwrap();
    stdout.flush().unwrap();
}
