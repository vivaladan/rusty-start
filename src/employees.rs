use std::collections::HashMap;
use std::io;
use std::io::Write;

struct Organisation {
    employees_by_department: HashMap<String, Vec<String>>
}

impl Organisation {
    fn new() -> Organisation {
        Organisation { 
            employees_by_department: HashMap::new()
        }
    }

    fn add(&mut self, input_iter: &mut std::str::SplitWhitespace) {
        if let Some(employee_name) = input_iter.next() {
            let preposition = input_iter.next();

            if preposition != Some("to") {
                println!("Missing preposition 'to'");
                return;
            }

            if let Some(department_name) = input_iter.next() {
                let department = self.employees_by_department
                    .entry(department_name.to_lowercase())
                    .or_insert(Vec::new());

                department.push(String::from(employee_name));

                println!("{} added to {}", employee_name, department_name);
            }
            else {
                println!("Missing department name");
                return;
            }
        }
        else {
            println!("Missing employee name");
            return;
        }        
    }

    fn list(&mut self, input_iter: &mut std::str::SplitWhitespace) {
        if let Some(department_name) = input_iter.next() {
            if let Some(department) = self.employees_by_department
                .get(&department_name.to_lowercase()) {
                    for employee_name in department {
                        println!("{}", employee_name);
                    }

            } else {
                println!("Unknown department");
                return;
            }
        }
        else {
            println!("Missing department name");
            return;
        }
    }
}

pub fn start_repl() {
    let mut organisation = Organisation::new();

    println!("Enter command, e.g.");
    println!("List");
    println!("List Engineering");
    println!("Add Sally to Engineering");
    println!();
    
    loop {
        let mut input = String::new();

        print!(": ");
        io::stdout().flush().expect("Failed to flush output"); 
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut input_iter = input.split_whitespace();
        let command = input_iter.next().expect("Invalid command").to_lowercase();

        if command == "list" {
            organisation.list(&mut input_iter);
        }
        if command == "add" {
            organisation.add(&mut input_iter);
        }
    }
}



// Using a hash map and vectors, create a text interface to allow a user to 
// add employee names to a department in a company. 
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or 
// all people in the company by department, sorted alphabetically.