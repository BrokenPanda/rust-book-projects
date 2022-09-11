fn main() {
    let x = 5;
    println!("outer x is {}", x);
    let x = x + 1;
    println!("outer x is {}", x);

    {
        let x = x + 2;
        println!("inner x is {}", x);
    }

    println!("outer x is {}", x);
}
