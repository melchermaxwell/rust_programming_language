fn main() {
    let mut s = String::from("hello world");

    let word = first_word_slice(&s);

    s.clear();

    //println!("The first word is: {}", word);
}

//this example highlights the issue that slices solve for
fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
// more advanced approach would take s: &str as parameter
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}
