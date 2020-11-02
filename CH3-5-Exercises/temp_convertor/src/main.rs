fn convert_to_c(temp_f:f32) -> f32{
    (temp_f - 32.0) * 5.0/9.0
}

fn main() {
    let temp_f = 212.0;
    let temp_c = convert_to_c(temp_f);
    println!("{}f is {}c", temp_f, temp_c)
}
