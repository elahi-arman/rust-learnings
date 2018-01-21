There are two different types for strings, the String type and the &str.
String is the dynamic heap string type -- use it to own or modify string data
str is an immutable sequence of UTF-8 bytes of dynamic length. Size is unknown, so can only be handled between a pointer. So a str is usually a reference to UTF data, a slice.

Use String if you need owned string data - passing strings to other tasks, or building at runtime. Use &str if we only need a view of a string

Since "Q" is a string literal, we need to match the type of scale from the String, to the underlying reference. So we can dereference using & and since we want the entire string, we use [..] to index it.

One interesting thing is the use of match, basically like a switch statement except shorter syntax. Looks good for one liners. 

``` rust
let mut scale = String::new();

io::stdin().read_line(&mut scale)
    .expect("Failed to read line");

//NOTE: always need to trim the read_lines from stdin
// the & converts from String -> &str and the [..] gets the full index
match &scale[..] {
    "Q" => break,
    "F" => println!("{}", temperature * 9 / 5 + 32),
    "C" => println!("{}", temperature - 32 * 5 / 9),
    _ => {
        println!("Couldn't understand that scale");
        continue;
    }
}
```
