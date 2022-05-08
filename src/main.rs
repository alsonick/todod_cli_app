use std::io;

struct Todo {
    title: String,
    completed: bool,
}

// Currently broken, fix later

fn main() {
    loop {
        let mut title = String::new();
        println!("Title:");
        io::stdin()
            .read_line(&mut title)
            .expect("Something went wrong.");
        
        if title.to_lowercase().trim() == String::from("stop")
        || title.to_lowercase().trim() == String::from("break") {
            println!("Stopping...");
            break;
        }

        let mut completed = String::new();
        println!("Have you completed this todo? ('yes' or 'no')");
        io::stdin()
            .read_line(&mut completed)
            .expect("Something went wrong");

        let mut completed_status = false;
        
        if completed == String::from("yes") {
            completed_status = true;
        }

        let todo = Todo {
            title,
            completed: completed_status
        };

        let mut todos = Vec::new();
        todos.push(todo);

        let mut list_todos_choice = String::new();
        println!("List out todos? ('yes' or 'no')");
        io::stdin()
            .read_line(&mut list_todos_choice)
            .expect("Something went wrong");

        if list_todos_choice.to_lowercase().trim() == String::from("yes") {
            for t in todos.iter() {
                println!("Title: {}", t.title);
                println!("Completed: {}", t.completed);
            }
        } else { continue; }

    }
}
