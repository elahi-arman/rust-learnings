fn main() {
    println!("Hello, world!");
}

fn shitty_first_word(s: &String) -> usize {
    // converts String to an array of bytes , then we can iterate by element
    let bytes = s.as_bytes();

    // create an iterator using iter method and enumerate returns a tuple with idx
    // since enumerate returns a tuple, we need to destructure it
    // enumerate returns a ref to the element, so we have to use &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    //returning this usize might not be a meaningful number in the future
    // like if clear was used on the string
    s.len()
}

// best to change this to (s: &str) -> &str since then we can operate on
// String - by passing a slice of the entire string -- &s[..]
// and &str since that's the type specified
fn better_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return a slice that's tied to the underlying data
            // then we let the compiler handle scoping
            return &s[0..i];
        }
    }

    &s[..]
}
