fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "Six"
    };
//
//     Compiling branches v0.1.0 (/media/LinuxStorageCryp/MyCurrentStorage/Coding/Rustbook-2020-10-17/branches)
// error[E0308]: `if` and `else` have incompatible types
// --> src/main.rs:20:9
//    |
// 17 |       let number = if condition {
//    |  __________________-
// 18 | |         5
//    | |         - expected because of this
// 19 | |     } else {
// 20 | |         "Six"
//    | |         ^^^^^ expected integer, found `&str`
// 21 | |     };
//    | |_____- `if` and `else` have incompatible types
//
// error: aborting due to previous error
//
// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `branches`.
//
// To learn more, run the command again with --verbose.

}
