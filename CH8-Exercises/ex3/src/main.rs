use std::collections::HashMap;


fn main() {
    let mut depts = HashMap::new();
    depts.insert(String::from("Accounting"), vec!["John Smith"]);
    depts.insert(String::from("IT"), Vec::new());
    depts.insert(String::from("Sales"), Vec::new());
    depts.insert(String::from("Ordering"), Vec::new());
    // depts.entry("Accounting".to_string()).push(String::from("John Smith"));
    let dept_name = String::from("Accounting");
    let dept = depts.get(&dept_name).unwrap();
    //dept.push("John Thompson");
    println!("{:?}", dept);
}
