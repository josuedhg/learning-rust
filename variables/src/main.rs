fn main() {
    let mut bunies = 16;
    let (bunnies, carrots) = (8, 50);
    const WARP_FACTOR: f64 = 9.9;

    print!("{} {} {}", bunies, carrots, WARP_FACTOR);

    bunies = 2;
    print!("{} {} {}", bunies, carrots, bunnies);
    
}
