fn do_day(day: i32) {
    println!("On the {} of Christmas my true love gave to me:", convert_day(day));
    let mut out: String = "".to_string();
    for count in (1..(day+1)).rev() {
        let text = match count {
            1 => "A partidge in a pear tree.",
            2 => "Two Turtle Doves, and",
            3 => "Three French Hens",
            4 => "Four Calling Birds",
            5 => "Five Golden Ring",
            6 => "Six Geese a-laying",
            7 => "Seven Sawns a-swimming",
            8 => "Eight Maids a-milking",
            9 => "Nine Ladies daning",
            10 => "Ten Lords a-leaping",
            11 => "Elven Pipers piping",
            12 => "Twelve Drummers drumming",
            _ => "To many days, Bonk to the head",
        };
        out.push_str(text);
        out.push_str("\n");
    }
    println!("{}",out);
}

fn convert_day(day: i32) -> String {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "Invalid",
    }.to_string()
}

fn main() {
    for day in 1..13 {
        do_day(day);
    }
}
