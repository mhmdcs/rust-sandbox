fn main() {
    // curly braces scopes {} are expressions that resokve to a value
    println!("{}", add_five({ 1 + 1 }));
}

fn add_five(x: i32) -> i32 {
    x + 5 // we can either return values at the end of curly braces like this, or with an explicit
    // return statement, also when we return expressions we must not add ; because the semicolon
    // turns an expression into a statement instead
}
