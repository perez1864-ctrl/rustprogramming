use std::io;
use std::process::Command;


enum FileOperation {
    List(String),           
    Display(String),        
    Create(String, String), 
    Remove(String),         
    Pwd,                   
}


fn perform_operation(operation: FileOperation) {
    match operation {

        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");

            if !status.success() {
                println!("Error listing directory.");
            }
        }

        FileOperation::Display(file) => {
            let status = Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                println!("Error displaying file.");
            }
        }

        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > {}", content, file);

            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File created successfully.");
            } else {
                println!("Failed to create file.");
            }
        }

        FileOperation::Remove(file) => {
            let status = Command::new("rm")
                .arg(file)
                .status()
                .expect("Failed to remove file");

            if status.success() {
                println!("File removed successfully.");
            } else {
                println!("Failed to remove file.");
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                println!("Error getting working directory.");
            }
        }
    }
}


fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn main() {

    println!("Welcome to the File Operations Program!");

    loop {

        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let choice = get_input("Enter your choice (0-5):");

        match choice.as_str() {

            "1" => {
                let path = get_input("Enter directory path:");
                perform_operation(FileOperation::List(path));
            }

            "2" => {
                let file = get_input("Enter file path:");
                perform_operation(FileOperation::Display(file));
            }

            "3" => {
                let file = get_input("Enter file path:");
                let content = get_input("Enter content:");
                perform_operation(FileOperation::Create(file, content));
            }

            "4" => {
                let file = get_input("Enter file path:");
                perform_operation(FileOperation::Remove(file));
            }

            "5" => {
                println!("Current working directory: ");
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}