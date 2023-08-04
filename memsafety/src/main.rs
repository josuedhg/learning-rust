fn main() {
    let enigma: i32;
    if true { // even when this is a literal true it is not going enigma is going to be consider as
        // uninitialized if there is not an else because of statement is evaluated at runtime
        enigma = 42;
    } else { // if we have an else statement we complete the initialization then enigma is consider
        // initialized variable and there is not error
        enigma = 42
    }
    println!("{}", enigma); // if else is not present this is going to get an error for posible
    // use of uninitialized variable
}
