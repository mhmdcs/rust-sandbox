fn main() {
    let mut count = 3;

    // while true {} is equivalent to loop {} but it can't return a value like loop {}
    while count != 0 {
        println!("Number is {count}");
        count -= 1;
    }

    println!("Stopped!");
}
