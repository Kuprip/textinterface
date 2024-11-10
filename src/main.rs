use std::collections::HashMap;

fn create_department(name: &str,db: &mut HashMap<String, Vec<String>>) {
    db.insert(name.to_string(), vec![String::from("")]);
}

fn add_user(name: &str, department: &str, db: &mut HashMap<String, Vec<String>>) {
    if !db.contains_key(department){
        create_department(department, db);
    }
    let v = db.get_mut(department).unwrap();
    let comp = vec![String::from("")];
    if v == &comp{
        let v = &mut vec![String::from(name)];
    } else {
        v.push(name.to_string());
    } 
    db.insert(department.to_string(), *v);
    
}


fn display_company(db: &HashMap<String, Vec<String>>){
    println!("{db:?}");
}




fn display_department(department: &str){

}

fn main() {
    let mut db = HashMap::new();
    let mut prompt = String::new();

    std::io::stdin().read_line(&mut prompt).expect("Failed to read line");
    let mut words = prompt.split(' ');
    let first = words.next().unwrap();
    let second = words.next().unwrap();
    let third = words.next().unwrap();
    if first == String::from("Add") || first == String::from("add") {
        add_user(second, third, &mut db);
    } else if first == String::from("Display "){
        if second == String::from("company"){
            display_company(&db);
        } else {
            display_department(second);
        }
    } else {
        println!("wrong command uwu!! ??!! !??!!");
    }
    println!("{db:?}")
}