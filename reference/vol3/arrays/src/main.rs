use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5];

    println!("Please enter an index number");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered is not a number");

    let element = array[index];

    println!("The value at index {index} is {element}");
}

fn arrays() {
    // Arrays are useful when you want your data allocated on the stack rather than the heap.
    // Or when you want to ensure you always have a fixed number of elements.
    // Use array if you know its size at compile time as it will be a fixed/static length.
    // And use vectors instaed if you want its size to be growable (resizable) i.e. grow or shrink in size.

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "November",
        "December",
    ];

    let arr_type_annotated: [i32; 5] = [0, 1, 2, 3, 4];

    let first = arr_type_annotated[0]; // first index value = 0
    let second = arr_type_annotated[1]; // second index value = 1 

    let arr_default_values = [3; 5]; // equal to [3, 3, 3, 3, 3] 
}
