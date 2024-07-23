fn main() {
    let mut x = 5;
    //Changing this variable to be a mut variable makes it so this program will run
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y in the outer scope is: {y}");

}
