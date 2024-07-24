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


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of Y is {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let array = [1, 2, 3, 4, 5];
    //This is another way to write an array, by specifing what type is in the array and how many elements are in the array too
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //A third way to write an array, this will make an array with length 5, filled only with 3's
    let b = [3; 5];
    
}
