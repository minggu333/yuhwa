// fn main() {
//     let a = 10;
//     println!("a = {}", a);
// }

// fn main() {
//     let a = 10;
//     if a > 5 {
//         println!("big");
//     }
// }

fn main() {
    let mut a = 10;
    for i in 0..10 {
        a = a + 5;
        println!("{}번째: a = {}", i + 1, a);
    }
}
