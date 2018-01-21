# Colllections

These collections are stored on the heap.

- vector => store variable number of values next to each other
- string => collection of characters
- hash map => key value pairs

# Vectors - https://doc.rust-lang.org/stable/nomicon/vec.html

Type is Vec <T>. Can only store values of the same Typ.

Constructor: ```let v: Vec<i32> = Vec::new();```

We need to have the type annotation since we didn't insert any values. Usually though we'll use the ```vec!``` macro to create vectors.

``` rust
let v = vec![1, 2, 3]
```

Dropping a vector drops its elements, which is tricky since we might have references in this vector.

## Methods
- push =>  add to the Vector, but the Vec needs to be mutable
- access => using the get method or the indexing syntax
    => indexing gives us a reference -- out of bounds causes panic!
    => get method gives us an Option<&T> -- out of bounds returns None
- iteration =>
    - for i in &v => immutable references
    - for i in &mut v => mutable references, then dereference i

Invalid References

``` rust
let mut v = vec![1, 2, 3, 4];
let first = &v[0];      // immutable borrow to vec elt
v.push(6);              // mutates the vector which is not allowed
```

## Extending Generics

Since Vectors can only be of one type. We can use enum types in the same way as a union.  

``` rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Since we know the exhaustive set of types that can be inside each Cell, we can use an enum. If we don't, the we'd have to use a trait object.

# Strings

To convert a &str to a String, use the .to_string() method or the String::from function.

## Methods
- appending
  - push_str -- append a string slice to a String
  - push -- append a char to a String
  - + -- combines two String values into a new value. Moves the lval, borrow immutable as rval
  - format! -- use this macro instead of multiple +
- indexing -- not supported since UTF8 can be represented as 1 or 2 byte u
- slicing -- interprets as bytes, but that can lead to invalid endings, so take care.
- iteration -- for c in String.chars or String.bytes()

## Internal Representation
A string is a wrapper over a Vec<u8>. It can be represented as grapheme clusters (letters), bytes, or scalar values.

# Hash Maps

Stores a mapping of keys of type K to value V. Need to be used from std::collections::HashMap. Can also use collect() on a vector of tuples


``` rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

is equivalent to

``` rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// use HashMap<_,_> since collect can be in many different data structure
// the _, _ so Rust can infer the types the map contains
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect()
```

For types with the Copy trait, they're copied into the hash map. Owned values are moved and the hash map becomes the owner. If references are inserted, then the references need to go out of scope when the HashMap goes out of scope. This can be protected using Lifetimes.

## Methods
- insert(K, V)
  - insert an already existing key
      - overwriting -- default behavior
      - only if key has no value -- entry(K).or_insert(V)
      - merge values -- get mutable reference using or_insert and then update the value. Each mutable reference goes out at the end of the current iteration
- get(K) => result is an option since that K might not exist in the map
- iteration => for (k, v) in &map
-
