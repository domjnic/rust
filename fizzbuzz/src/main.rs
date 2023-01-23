fn main() {
    let mut x = 0;

    for x in 1..101 {       
        if x % 15 == 0 {
            println!("FizzBuzz")
        }
        else if x % 3 == 0 {
            println!("Fizz");
        }
        else if x % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{x}");
        }
    }
}
