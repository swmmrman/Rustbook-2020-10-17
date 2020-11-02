fn convert_to_c(temp_f:f32) -> f32{
    (temp_f - 32.0) * 5.0/9.0
}

fn convert_to_f(temp_c:f32) -> f32 {
    (temp_c * 9.0/5.0) + 32.0
}

fn main() {
    let temp_f = 212.0;
    let temp_c = convert_to_c(temp_f);
    println!("{}f is {}c", temp_f, temp_c);
    let temp_c = 100.0;
    let temp_f = convert_to_f(temp_c);
    println!("{}c is {}f", temp_c, temp_f);
}
