use std::{env,process};

fn mutate(word: &str) -> String {
    let vowels = vec!["a", "e", "i", "o", "u", "y"];
    let small = word.to_lowercase();        // Change string to lowercase.
    let firstletter = &small[0..1];         // Pull off first letter.
    let results;                            // Decalre storage space
    if vowels.contains(&firstletter) {      // add hay to ones starting in vowels.
        results = format!("{}-hay", &word);
    }
    else {                                  // rearrange the others.
        results = format!("{}-{}ay", &word[1..], firstletter);
    }
    results
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Please wrap you sentence in quotes. Ex. \"This way\"");
        process::exit(1);
    }
    let sentence = &args[1];
    let mut newsentence = String::new();
    for word in sentence.split(" ") {
        newsentence = format!("{} {}",&newsentence, mutate(&word));
    }
    println!("{}", newsentence);
}
