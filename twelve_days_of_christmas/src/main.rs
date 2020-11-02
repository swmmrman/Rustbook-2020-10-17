fn do_day(day: i32) {
    println!("On the {} of Christmas my true love gave to me:", day);
    let mut out: String = "".to_string();
    for count in (1..(day+1)).rev() {
        let text = match count {
            1 => "A partidge in a pear tree.",
            2 => "2 turtle doves, and",
            3 => "3 French Hens",
            4 => "4 Calling Birds",
            5 => "5 Golden Ring",
            6 => "6 Geese a-laying",
            7 => "7 Sawns a-swimming",
            _ => "Invalid day, Bonk to the head.",
        };
        out.push_str(text);
        out.push_str("\n");
    }
    println!("{}",out);
}

fn main() {
    for day in 1..13 {
        do_day(day);
    }
}
