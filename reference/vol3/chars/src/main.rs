fn main() {
    let char_a = 'a';
    let byte_a = b'a';

    let ascii: char = 'A';
    let emoji: char = '🔥';

    println!("Size of ASCII char a: {} bytes", mem::size_of_val(&char_a));
    println!("Size of ASCII byte a: {} bytes", mem::size_of_val(&byte_a));

    println!("Size of ASCII char: {} bytes", mem::size_of_val(&ascii));
    println!("Size of Emoji char: {} bytes", mem::size_of_val(&emoji));
}
