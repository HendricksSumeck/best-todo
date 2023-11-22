use std::process::{Command};
use std::fs::{OpenOptions};
use std::io::prelude::Read;
use std::io::{BufReader, BufWriter, Write};
use std::{process};
use dialoguer::{theme::ColorfulTheme, Select, Input};

pub struct Todo {
    pub todo_path: String,
}

impl Todo {
    pub fn new(path: String) -> Result<Self, String> {
        let todo_path: String = String::from(path);

        if !file_exists(&todo_path) {
            match create_file(&todo_path) {
                Ok(_) => {
                    println!("File created: {}", &todo_path);
                }
                Err(err) => {
                    return Err(format!("Error creating file: {}", err));
                }
            }
        }

        Ok(Self {
            todo_path
        })
    }

    fn get_todo_list(&self) -> Vec<String> {
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buf_reader = BufReader::new(&todofile);

        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let todo = contents.lines().map(str::to_string).collect();

        todo
    }

    pub fn list(&self) {
        let mut data = String::new();
        let mut list_data: Vec<String> = Vec::new();

        let todo = self.get_todo_list();

        for (number, task) in todo.iter().enumerate() {
            if task.len() > 5 {
                let number = (number + 1).to_string();

                let symbol = &task[..4];
                let task = &task[4..];

                if symbol == "[*] " {
                    data = format!("{} [*] {}", number, task);
                } else if symbol == "[ ] " {
                    data = format!("{} [ ] {}", number, task);
                }
                list_data.push(data.to_string());
            }
        }
        list_data.push("Back".to_string());

        let slice_data: &[String] = list_data.as_slice();
        let selection = menu(slice_data, "Choose an option:".to_string());

        let arg = list_data.get(selection).unwrap();

        let mut back = true;
        match &arg[..] {
            "Back" => back = false,
            _ => { }
        }

        if back {
            let items: Vec<String> = vec![
                String::from("Conclude"),
                String::from("To remove"),
                String::from("Back"),
            ];

            let data: &[String] = items.as_slice();

            let selection2 = menu(data, "Choose an option:".to_string());

            let command = items.get(selection2).unwrap();

            match &command[..] {
                "Conclude" => self.done(arg),
                "To remove" => self.remove(arg),
                "Back" => { }
                _ => { }
            }
        }
        clear_terminal_screen()
    }

    pub fn add(&self) {

        let args: String = Input::new()
            .with_prompt("New to do")
            .interact_text()
            .unwrap();

        if args.is_empty() {
            eprintln!("Todo add takes at least 1 argument");
            process::exit(1);
        }

        let todofile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);


        let line = format!("[ ] {}\n", args);
        buffer
            .write_all(line.as_bytes())
            .expect("Unable to write data");

        clear_terminal_screen()
    }

    pub fn remove(&self, args: &String) {
        let todo = self.get_todo_list();

        let todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in todo.iter().enumerate() {
            let first = args.chars().next();
            let value = (pos + 1).to_string().chars().next();

            if Some(first) == Option::from(value) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("Unable to write data");

            buffer.flush().expect("Unable to flush buffer");
        }
        clear_terminal_screen()
    }

    pub fn done(&self, arg: &str) {
        let todofile = OpenOptions::new()
            .write(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let todo = self.get_todo_list();

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in todo.iter().enumerate() {
            if line.len() > 5 {
                let first = arg.chars().next();
                let value = (pos + 1).to_string().chars().next();

                if Some(first) == Option::from(value) {
                    if &line[..4] == "[ ] " {
                        let line = format!("[*] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    }
                } else if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    buffer
                        .write_all(line.as_bytes())
                        .expect("unable to write data");
                }
            }
        }
        clear_terminal_screen()
    }
}

fn menu(data: &[String], prompt: String) -> usize {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(data)
        .interact()
        .unwrap();

    selection
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

fn create_file(path: &str) -> Result<(), std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;

    Ok(())
}