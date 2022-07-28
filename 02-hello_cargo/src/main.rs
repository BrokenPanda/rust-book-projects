fn main() {
    println!("Hello, world!");
    println!("{}", is_even(5));
}

pub fn is_even(num: u8) -> bool {
    (num%2) == 0
}