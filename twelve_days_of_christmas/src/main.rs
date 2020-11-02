fn do_day(day: i32) {
    println!("On the {} of Christmas my true love gave to me", day);
    let mut out: String = "".to_string();
    for count in (1..(day+1)).rev() {
        match count {
            1 => out.push_str("A partidge in a pear tree."),
            2 => out.push_str("2 turtle doves, and\n"),
            3 => out.push_str("3 French Hens\n"),
            _ => println!("WTF"),
        }
    }
    println!("{}\n",out);
}

fn main() {
    for day in 1..4 {
        do_day(day);
    }
}
