use std::collections::HashMap;
mod db;

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

        if first == String::from("Add") {
            db::add_user(second, third, &mut db);
        } else if first == String::from("Display"){
            if second == String::from("company"){
                db::display_company(&db);
            } else if second == String::from("department"){
                db::display_department(third, &db);
            }
        } else {
            println!("wrong command uwu!! ??!! !??!!");
        }
    }
}