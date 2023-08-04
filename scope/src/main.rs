fn main() {
    let mut x = 5;
    {
        let x = 99;
        let y = 21;
        println!("{}, {}", x, y);
    }

    println!("{}", x);

    x = 10;
    let x = x;
    println!("{}", x);
}
