use rand::thread_rng;
use rand::Rng;

pub fn greeting() {
    let x = thread_rng().gen_range(0, 100);
    println!("Hello world {}!", x);
}
