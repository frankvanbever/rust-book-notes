use std::collections::HashMap;

// Using a hash map and vectors, create a text interface that
// allows a user to add employee names to a department in a company;
// for example, "Add Sally to Engineering" or "Add Amir to Sales".
// Then let the user retrieve a list of all people in a department or
// all people in the company by department sorted alphabetically.

fn add_employee(departments: &mut HashMap<&str, Vec<String>>, command: String) {

    let mut tokens = Vec::new();

    for token in command.split_whitespace() {
        tokens.push(token);
    }

    if tokens.len() == 4 && tokens[0] == "Add" && tokens[2] == "to" {
        let department = departments.get_mut(tokens[3]).unwrap();

        let name = String::from(tokens[1]);
        department.push(name);
    } else {
        println!("Malformed command");
    }
}

fn print_department(departments: &HashMap<&str, Vec<String>>, department_name: &str) {
    let department = departments.get(department_name).unwrap();

    println!("{department_name}:");
    for name in department {
        println!("- {name}");
    }
}

fn main() {
    let mut departments = HashMap::new();

    let engineering:  Vec<String> = Vec::new();
    let sales: Vec<String> = Vec::new();

    departments.insert("Engineering", engineering);
    departments.insert("Sales", sales);

    add_employee(&mut departments, String::from("Add Sally to Engineering"));
    print_department(&departments, "Engineering");

    add_employee(&mut departments, String::from("Add Amir to Sales"));
    print_department(&departments, "Sales");
}
