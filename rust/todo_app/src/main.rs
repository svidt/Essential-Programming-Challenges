use std::collections::HashMap;

struct TodoList {
    items: HashMap<String, bool>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            items: HashMap::new()
        }
    }

    fn add(&mut self, task: String) {
        self.items.insert(task, false);
    }

    fn complete(&mut self, task: &str) {
        if let Some(status) = self.items.get_mut(task) {
            *status = true;
        }
    }

    fn list(&self) {
        for (task, &completed) in &self.items {
            println!("[{}] {}", if completed { "x" } else { " " }, task);
        }
    }
}

use std::io::{self, Write};

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print!("Enter a command (add/complete/list/quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "add" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
            }
            "complete" => {
                print!("Enter task to complete: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todo_list.complete(task.trim());
            }
            "list" => todo_list.list(),
            "quit" => break,
            _ => println!("Unknown command")
        }
    }
}