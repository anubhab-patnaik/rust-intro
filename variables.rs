fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 19;
    println!("{}", x);
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
    println!("y is {}", y);
    let x = x + 1;
    println!("x is {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
    // const fdfdf: u32 = 3;
    // println!("The value of fdfdf is {}", fdfdf);
}
