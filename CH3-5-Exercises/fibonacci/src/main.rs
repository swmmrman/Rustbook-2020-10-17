fn main() {
    let mut num = 1;
    let mut prev_num = 0;

    while num < 50 {
        let new_num = prev_num + num;
        prev_num = num;
        num = new_num;
        print!("{},", num);
    }
}
