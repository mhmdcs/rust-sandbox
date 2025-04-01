fn main() {
    let mut counter = 0;

    // loop {} construct is an expression that can resolve to a value (i.e. return a value) whereas
    // while condition {} can't.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Returned result from loop is {result}");
}
