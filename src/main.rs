use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use chrono::Local;

const FILE_PATH: &str = "todo.txt";

enum Command {
    Add(String),
    List(bool),
    Done(String),
    Delete(String),
    Save,
    Exit
}

#[derive(Debug, Clone)]
struct Task {
    timestamp: String,
    title: String,
    is_completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
        }
    }

    fn load(&mut self) {
        if Path::new(FILE_PATH).exists() {
            let file = File::open(FILE_PATH).unwrap();
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line.unwrap();
                let line_first = line.chars().nth(3).unwrap();
                let line = line[6..].split(':').collect::<Vec<&str>>();
                let task = Task {
                    timestamp: line[0].trim().to_string(),
                    title: line[1..].join(" ").trim().to_string(),
                    is_completed: line_first == 'x',
                };
                self.tasks.push(task);
            }
        }
    }
    
    fn save(&self) {
        let mut file = File::create(FILE_PATH).expect("Failed to create file");
        for task in &self.tasks {
            let status = if task.is_completed { "x" } else { " " };
            writeln!(file, "- [{}] {}: {}", status, task.timestamp, task.title).expect("Failed to write to file");
        }        
    }

    fn check_list(&self, title: &str) -> bool {
        self.tasks.iter().any(|x| x.title == title)
    }

    fn add_task(&mut self, title: String) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        if self.check_list(&title) {
            println!("task {} is already exists", title);
            return;
        }
        println!("add task: {} ({})", title, now);
        let task = Task {
            timestamp: now,
            title: title,
            is_completed: false
        };
        self.tasks.push(task);
    }

    fn list(&self, is_all: bool) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }
        let mut count = 0;
        for task in &self.tasks {
            if !is_all && task.is_completed {
                continue;
            }
            let status = if task.is_completed { 'x' } else { ' ' };
            println!("- [{}] {}: {}", status, task.title, task.timestamp);
            count += 1;
        }
        if count == 0 {
            println!("No tasks found. The command to open completed task is -all.");
        }
    }

    fn done(&mut self, title: String) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.iter_mut().find(|x| x.title == title) {
            task.is_completed = true;
            println!("task {} is done", task.title);
            Ok(())
        } else {
            Err("Task not found.")
        }
    }

    fn delete(&mut self, title: String) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.iter().position(|x| x.title == title) {
            let removed_task = self.tasks.remove(task);
            println!("task {} is deleted", removed_task.title);
            Ok(())
        } else {
            Err("Task not found.")
        }
    }
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts = input.split_whitespace();
    let cmd = parts.next().ok_or("No command provided")?;
    let args: Vec<&str> = parts.collect();

    match cmd {
        "add" => {
            if args.is_empty() {
                Err("No title provided for add command".to_string())
            } else {
                Ok(Command::Add(args.join(" ")))
            }
        }
        "list" => {
            if args.is_empty() {
                Ok(Command::List(false))
            } else if args.len() == 1 && args[0] == "-all" {
                Ok(Command::List(true))
            } else {
                Err("Invalid argument for list command".to_string())
            }
        }
        "done" => {
            if args.is_empty() {
                Err("No ID provided for done command".to_string())
            } else {
                Ok(Command::Done(args.join(" ")))
            }
        }
        "del" => {
            if args.is_empty() {
                Err("No ID provided for delete command".to_string())
            } else {
                Ok(Command::Delete(args.join(" ")))
            }
        }
        "save" => Ok(Command::Save),
        "exit" => Ok(Command::Exit),
        _ => Err(format!("Unknown command: {}", cmd))
    }
}

fn main() {
    let mut todo = TodoList::new();
    println!("todo.txtをloadしますか？（y/n）: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "y" {
        todo.load();
    }
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let command = match parse_command(input) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("err -> {}", e);
                continue;
            }
        };

        match command {
            Command::Add(title) => todo.add_task(title),
            Command::List(show_all) => todo.list(show_all),
            Command::Done(id) => {
                if let Err(e) = todo.done(id) {
                    println!("err -> {}", e);
                }
            }
            Command::Delete(id) => {
                if let Err(e) = todo.delete(id) {
                    println!("err -> {}", e);
                }
            }
            Command::Save => {
                todo.save();
                println!("Tasks saved to {}", FILE_PATH);
            }
            Command::Exit => {
                todo.save();
                println!("Tasks saved to {}", FILE_PATH);
                println!("Exiting...");
                break;
            }
        }
    }
}