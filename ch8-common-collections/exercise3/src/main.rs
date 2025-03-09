use std::collections::HashMap;

// Using a hash map and vectors, create a text interface that
// allows a user to add employee names to a department in a company;
// for example, "Add Sally to Engineering" or "Add Amir to Sales".
// Then let the user retrieve a list of all people in a department or
// all people in the company by department sorted alphabetically.

// String references for the keys need to live as long as the
// Company hashmap
type Company<'a> = HashMap<&'a str, Vec<String>>;

fn add_employee(departments: &mut Company, command: String) {

    let mut tokens = Vec::new();

    for token in command.split_whitespace() {
        tokens.push(token);
    }

    if tokens.len() == 4 && tokens[0] == "Add" && tokens[2] == "to" {
        let department = departments.get_mut(tokens[3]).unwrap();

        let name = String::from(tokens[1]);
        department.push(name);
        department.sort(); // This is wasteful
    } else {
        println!("Malformed command");
    }
}

fn print_department(departments: &Company, department_name: &str) {
    let department = departments.get(department_name).unwrap();

    println!("{department_name}:");
    for name in department {
        println!("- {name}");
    }
}

fn print_company(departments: &Company) {
    for (department, _value) in departments {
       print_department(departments, department);
    }
}

fn main() {
    let mut departments = HashMap::new();

    let engineering:  Vec<String> = Vec::new();
    let sales: Vec<String> = Vec::new();

    departments.insert("Engineering", engineering);
    departments.insert("Sales", sales);

    add_employee(&mut departments, String::from("Add Amir to Sales"));
    print_department(&departments, "Sales");

    // Check if sorting actually works
    add_employee(&mut departments, String::from("Add Sally to Engineering"));
    add_employee(&mut departments, String::from("Add Art to Engineering"));
    add_employee(&mut departments, String::from("Add George to Engineering"));
    print_department(&departments, "Engineering");

    print_company(&departments);
}
