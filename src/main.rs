//Function that adds 5 to a variable a, 10 times, and prints the value of a each time.
fn main() {
    let mut a = 10;
    for i in 0..10 {
        a = a + 5;
        println!("{}번째: a = {}", i+1, a);
    }
}