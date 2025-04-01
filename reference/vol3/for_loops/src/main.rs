fn main() {
    let a = [5, 7, 2, 5, 9];

    // when looping through a collection like arrays, iterating through them with `for {}` is much
    // more performant and safer than manually checking the index with `while {}` due to the fact
    // that `while` performs bound checks at runtime at every iteration which is costly, whereas
    // `for` due to the `iterator protoocl` it avoids manual index checking at runtime due to
    // Rust's optimizer being able to prove at compile time that the iteration stays at within
    // bounds; iterators never expose the index, they just move to the next item in the collection
    // until there's nothing left.
    for element in a {
        println!("element is {element}");
    }
}
