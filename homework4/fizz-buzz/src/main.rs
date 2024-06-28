fn fizzBuzz() {
    for i in 1..=301 {
        match (i%3 == 0,  i%5 == 0) {
            (true, true) => println!("fizz buzz"),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (false, false) => (),
        }
        
    }
}

fn main() {
    fizzBuzz();
}
