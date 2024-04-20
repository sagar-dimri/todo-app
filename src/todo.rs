use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader, BufWriter};

pub struct Todo {
    data: Vec<String>,
    path: String,
    path_bak: String
}

impl Todo {
    pub fn new() -> Todo{
        Todo{
            data: Vec::new(),
            path: "./todo.txt".to_string(),
            path_bak: "./todo_bak.txt".to_string()
        }
    }

    pub fn add(&self, items: &[String]) {
        // Open a file with append option
        let mut data_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("data.txt")
            .expect("cannot open file");

        let mut writer = BufWriter::new(data_file);
        for line in items {
            writeln!(writer, "[] {}", line).unwrap();
        }

        writer.flush().unwrap();
    }

    pub fn list(&self) {
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("data.txt")
            .expect("cannot open file");

        let reader = BufReader::new(data_file);

        for line in reader.lines(){
            match line {
                Ok(line_content) => {
                    // println!("{}", line_content)
                    if line_content.starts_with("[*]") {
                        println!("\x1B[9m{}\x1B[0m", &line_content[3..]);
                    }else {
                        println!("{}", &line_content[3..]);
                    }
                },
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    pub fn done(&mut self, items: &[u8]) {
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("data.txt")
            .expect("cannot open file");

        let reader = BufReader::new(data_file);

        for (_, line) in reader.lines().enumerate() {
            match line {
                Ok(line_content) => {
                    (self.data).push(line_content);
                },
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }

        let data_file_w = OpenOptions::new()
            .write(true)
            .create(true)
            .open("data.txt")
            .expect("cannot open file");

        let mut writer = BufWriter::new(data_file_w);

        for entry in items {
            if *entry as usize <= self.data.len() {
                let idx = (*entry-1) as usize;
                self.data[idx].replace_range(..3, "[*]");
            }
        }

        for line in self.data.iter() {
            writeln!(writer, "{}", line).unwrap();
        }

        // flush the writer buffer
        writer.flush().unwrap();
        // clean up the vector<String> as well
        while let Some(_) = self.data.pop() {}
    }

    pub fn reset(&self) {
        let source_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./data.txt")
            .expect("couldn't create / truncate file");
        let mut source_reader = BufReader::new(source_file);
        let dest_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./data_bak.txt")
            .expect("couldn't create / truncate file");
        let mut dest_writer = BufWriter::new(dest_file);

        io::copy(&mut source_reader, &mut dest_writer).unwrap();

        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./data.txt")
            .expect("couldn't truncate file");
    }

    pub fn restore(&self) {
        let source_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./data_bak.txt")
            .expect("couldn't open file");
        let mut source_reader = BufReader::new(source_file);

        let dest_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("./data.txt")
            .expect("couldn't open file");
        let mut dest_writer = BufWriter::new(dest_file);

        io::copy(&mut source_reader, &mut dest_writer).unwrap();
    }

    pub fn remove(&mut self, indices: &[u8]) {
        let mut new_indices_vec = Vec::new();
        for index in indices {
            new_indices_vec.push(*index);
        } 
        let indices = &mut new_indices_vec[..];
        
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("data.txt")
            .expect("cannot open file");

        let reader = BufReader::new(data_file);

        for (_, line) in reader.lines().enumerate() {
            match line {
                Ok(line_content) => {
                    (self.data).push(line_content);
                },
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }

        indices.sort_by(|a, b| a.cmp(b));

        for idx in indices.iter().rev() {
            if *idx as usize <= self.data.len() {
                self.data.remove((*idx as usize) - 1);
            }
        }

        let data_file_w = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("data.txt")
            .expect("cannot open file");

        let mut writer = BufWriter::new(data_file_w);

        for line in self.data.iter() {
            writeln!(writer, "{}", line).unwrap();
        }

        // flush the writer buffer
        writer.flush().unwrap();
        // clean up the vector<String> as well
        while let Some(_) = self.data.pop() {}
    }

    pub fn sort(&self) {
        unimplemented!();
    }
    
    pub fn help(&self) {
        const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
        Todo is a super fast and simple tasks organizer written in rust
        Example: todo list
        Available commands:
            - add [TASK/s]
                adds new task/s
                Example: todo add \"buy carrots\"
            - list
                lists all tasks
                Example: todo list
            - done [INDEX]
                marks task as done
                Example: todo done 2 3 (marks second and third tasks as completed)
            - rm [INDEX]
                removes a task
                Example: todo rm 4
            - reset
                deletes all tasks
            - restore 
                restore recent backup after reset
        ";
        println!("{}", TODO_HELP);
    }
}