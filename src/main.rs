use std::collections::HashMap;

fn create_department(name: &str,db: &mut HashMap<String, Vec<String>>) {
    db.insert(name.to_string(), Vec::new());
}

fn add_user(name: &str, department: &str, db: &mut HashMap<String, Vec<String>>) {
    if !db.contains_key(department){
        create_department(department, db);
    }
    let v = db.get_mut(department).unwrap();
    v.push(name.to_string());
}


fn display_company(db: &HashMap<String, Vec<String>>){
    println!("{db:?}");
}




fn display_department(department: &str, db: &HashMap<String, Vec<String>>){
    let department_staff = db.get_key_value(department);
    println!("{department_staff:?}");
}

fn main() {
    let mut db = HashMap::new();
    loop {
        let mut prompt = String::new();
        std::io::stdin().read_line(&mut prompt).expect("Failed to read line");
        prompt.pop(); // pops the \n
        let mut words = prompt.split(' ');
        let first = words.next().unwrap();
        let second = words.next().unwrap();
        let third = words.next().unwrap();

        if first == String::from("Add") || first == String::from("add") {
            add_user(second, third, &mut db);
        } else if first == String::from("Display"){
            if second == String::from("company"){
                display_company(&db);
            } else if second == String::from("department"){
                display_department(third, &db);
            }
        } else {
            println!("wrong command uwu!! ??!! !??!!");
        }
    }
}