fn convert_to_c(temp_f:f32) -> f32{
    (temp_f - 32.0) * 5.0/9.0
}

fn convert_to_f(temp_c:f32) -> f32 {
    (temp_c * 9.0/5.0) + 32.0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let &temp = &args[2].parse().unwrap();
    let switch = &args[1][0..];
    match switch {
        "-c" => {
            println!("Result {}c", convert_to_c(temp));
        },
        "-f" => {
            println!("Result {}f", convert_to_f(temp));
        },
        _ => println!("No conversion specified"),
    }
}
