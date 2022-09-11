fn main() {
    println!("Hello, world!");
    my_function(2, 'A');
}

fn my_function(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}