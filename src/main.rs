use std::io::{self, Write};

enum Command {
    Add(String),
    List(bool),
    Done(String),
    Delete(String),
    Exit
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    title: String,
    is_completed: bool,
    deleted: bool
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1
        }
    }

    fn add_task(&mut self, title: String) {
        let task = Task {
            id: self.next_id,
            title,
            is_completed: false,
            deleted: false
        };
        self.tasks.push(task);
        println!("add task {}", self.tasks.get(self.next_id - 1).unwrap().title);
        self.next_id += 1;
    }

    fn list(&self, is_all: bool) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }
        let mut count = 0;
        for task in &self.tasks {
            if task.deleted {
                continue;
            }
            if !is_all && task.is_completed {
                continue;
            }
            let status = if task.is_completed { "Completed" } else { "Pending" };
            println!("{}: {} [{}]", task.id, task.title, status);
            count += 1;
        }
        if count == 0 {
            println!("No tasks found. The command to open completed task is -all.");
        }
    }

    fn done(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.iter_mut().find(|x| x.title == id) {
            task.is_completed = true;
            println!("task {} is done", task.title);
            Ok(())
        } else {
            Err("Task not found.")
        }
    }

    fn delete(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.iter_mut().find(|x| x.title == id && !x.deleted) {
            task.deleted = true;
            println!("task {} is deleted", task.title);
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
            let id: String = args.first().ok_or("No ID provided for done command")?.to_string();
            Ok(Command::Done(id))
        }
        "del" => {
            let id: String = args.first().ok_or("No ID provided for delete command")?.to_string();
            Ok(Command::Delete(id))
        }
        "exit" => Ok(Command::Exit),
        _ => Err(format!("Unknown command: {}", cmd))
    }
}


fn main() {
    let mut todo = TodoList::new();
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
            Command::Exit => {
                println!("Exiting...");
                break;
            }
        }
    }
}