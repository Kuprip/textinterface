use std::collections::HashMap;

fn create_department(name: &str,db: &mut HashMap<String, Vec<String>>) {
    db.insert(name.to_string(), Vec::new());
}

pub fn add_user(name: &str, department: &str, db: &mut HashMap<String, Vec<String>>) {
    if !db.contains_key(department){
        create_department(department, db);
    }
    let v = db.get_mut(department).unwrap();
    v.push(name.to_string());
}

pub fn display_company(db: &HashMap<String, Vec<String>>){
    println!("{db:?}");
}

pub fn display_department(department: &str, db: &HashMap<String, Vec<String>>){
    let department_staff = db.get_key_value(department);
    println!("{department_staff:?}");
}
