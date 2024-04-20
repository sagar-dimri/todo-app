use crate::todo::Todo;

mod todo;

use std::env;

fn main() {
    let mut todo = Todo::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => {
                let mut bytes = Vec::new();
                for s in &args[2..] {
                    let val = s.parse::<u8>().unwrap();
                    bytes.push(val);
                }
                let byte_slice: &[u8] = &bytes;
                todo.remove(byte_slice);
            },
            "done" => {
                let mut bytes = Vec::new();
                for s in &args[2..] {
                    let val = s.parse::<u8>().unwrap();
                    bytes.push(val);
                }
                let byte_slice: &[u8] = &bytes;
                todo.done(byte_slice);
            },
            // "raw" => todo.raw(&args[2..]),
            // "sort" => todo.sort(),
            "reset" => todo.reset(),
            "restore" => todo.restore(),
            "help" | "--help" | "-h" | _ => todo.help(),
        }
    } else {
        todo.list();
    }
}
