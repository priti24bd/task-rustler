use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    loop {
        println!("\nTodo App: ");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Delete task");
        println!("4. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut task = String::new();
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut task).unwrap();
                tasks.push(task.trim().to_string());
            },
            "2" => {
                println!("\nTasks:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}: {}", i + 1, task);
                }
            },
            "3" => {
                let mut index = String::new();
                print!("Enter task number to delete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut index).unwrap();
                if let Ok(num) = index.trim().parse::<usize>() {
                    if num > 0 && num <= tasks.len() {
                        tasks.remove(num - 1);
                        println!("Task removed!");
                    } else {
                        println!("Invalid task number!");
                    }
                }
            },
            "4" => break,
            _ => println!("Invalid option, try again!"),
        }
    }
}
