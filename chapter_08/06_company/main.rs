use std::io;
use std::collections::HashMap;

fn display_commands() {
    println!("List of available commands: ");
    println!("    Add \"name\" to \"deparment\": adds person with the given name to the given department.");
    println!("    List \"department\": Lists every person that works at the given department.");
    println!("    List All: Lists every person that works at the company.");
    println!("    Exit: Exists the program.");
    println!("    Help: Displays lists of available commands.");
}

fn insert_new_employee(
    company: &mut HashMap<String, Vec<String>>, 
    name: Option<&str>, 
    department: Option<&str>
) {
    if let Some(i) = name {
        let new_name = String::from(i);
        if let Some(j) = department {
            let new_department = String::from(j);
            let department_vector = company.entry(new_department).or_insert(Vec::new());
            department_vector.push(new_name);
            println!("New employee added succesfully!")
        } else {
            println!("No department for the new employee was provided.");
        }
    } else {
        println!("No name for the new employee was provided.");
    }
}

fn list_people_in_department(department: &Vec<String>) {
    if department.is_empty() {
        println!("    No people assign to this department.");
    }
    else {
        for employee in department {
            println!("    - {employee}");
        }
    }
}

fn list_all_people(company: &mut HashMap<String, Vec<String>>) {
    for (key, value) in company {
        println!("{key}: ");
        value.sort();
        list_people_in_department(value);
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Please input a command, or use Help to get a list of available commands.");
    
    loop {
        let mut input_string = String::new();
        
        io::stdin().read_line(&mut input_string).expect("Failed to read the command");

        let mut iter = input_string.split_whitespace();
        
        if let Some(word) = iter.next() {
            match word.to_lowercase().as_str() {
                "add" => {
                    let name = iter.next();
                    let _ = iter.next();
                    let department = iter.next();
                    insert_new_employee(&mut company, name, department);    
                },
                "list" => {
                    if let Some(detail) = iter.next() {
                        match detail.to_lowercase().as_str() {
                            "all" => list_all_people(&mut company),
                            _ => {
                                println!("{detail}:");
                                list_people_in_department(&company.get(detail).unwrap_or(&Vec::new()));
                            },
                        }
                    } else {
                        println!("No option was given for the list command.");
                    }
                },
                "help" => {
                    display_commands();
                },
                "exit" => {
                    break;
                },
                _ => {
                    println!("Command not recognize.");
                    display_commands();
                }
            }
        } else {
            println!("No command was provided!");
        }
    }
}