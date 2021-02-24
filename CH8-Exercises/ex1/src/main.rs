fn main() {
    let integers = vec![32, 18, 11, 18, 6, 66, 56, 5, 29, 61, 95, 61, 22, 52, 90, 52, 65, 13, 87, 12, 21, 86, 5, 15, 4, 77, 78, 62, 66, 64, 84, 18, 46, 90, 24, 22, 41, 37, 55, 71, 68, 13, 30, 27, 85, 5, 58, 54, 80, 19, 88, 97, 25, 75, 68, 60, 11, 33, 17, 7, 51, 74, 53, 16, 44, 70, 74, 14, 86, 50, 27, 5, 82, 91, 11, 63, 52, 61, 15, 16, 92, 73, 7, 93, 51, 84, 87, 43, 12, 97, 2, 32, 73, 91, 13, 20, 3, 57, 49, 87];
    let mut len = 0;
    let mut sum = 0;
    for i in integers.clone(){
        len += 1;
        sum += i;
    }
    let mut ordered = integers.to_vec();
    ordered.sort();
    println!("The integers are:{:?}\n\n", integers);
    let mean = sum / len;
    let mid = len/2;
    let median = ordered[mid];
    println!("Sum = {}, length = {}, Mean = {}, Median = {:?}", sum, len, mean, median);
}
