fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("Value of x is: {x}");

    let x = 6;
    println!("value of x id: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
}
