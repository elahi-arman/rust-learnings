Memory is managed through system of ownership with a set of rules that the compiler checks at compile time. So no run time costs incurred for ownership features.

Background Information
---
The stack is fast because of the way it accesses the data: it never has to search for a place to put new data or a place to get data from because that place is always the top. Another property that makes the stack fast is that all data on the stack must take up a known, fixed size.

Accessing data in the heap is slower than accessing data on the stack because we have to follow a pointer to get there. By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so we don’t run out of space are all problems that ownership addresses.

Managing heap data is why ownership exists.


Ownership Rules
---
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

``` rust
let s = String::from("hello");  // creating a String from a string literal, :: is for namespacing
```

String literals - known at compile time, super fast and efficient
String type, to support mutability and growability needs to be on the heap
  String::from -- our allocation, the deallocation happens when a variable out of scope.
  Rust calls the drop function automatically to do this for us -- this is RAII

``` rust
let s1 = String::from("hello");
let s2 = s1;        // this creates a new reference on stack to heap data
```

When that assignment happens, Rust automatically invalidates s1 to prevent a double free. This is a move instead of a shallow copy due to the invalidation. Rust will never automatically create “deep” copies of your data.
To deep copy this data, we would use ```clone()```.

Rust uses the Copy trait for copying on the stack. Since exact specs are known at compile time, these copies are always a deep copy. If a type implements Drop, then it cannot implement Copy. Generally scalar types like u32, bool, char, f64, and tuples of scalar types can implement Copy.

``` Rust
fn main() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.
  ```

  Using s after the take_ownership function would be a compiler error. But returning a value can transfer ownership. Ownership of a variables always follows the same pattern - assigning a value to another variables moves it. Rust lets you return multiple values using a tuple. But there's too much ceremony if we have to always transfer ownership to and from a function. So that's where we use references.

References
---
Defined by ampersands and allow you to refer to some value without taking ownership of it. Deferencing is done using `*`, so just like C.

``` rust
let s1 = String::from("hello");
let len = calculate_length(&s1); //&s1 creates a reference but no ownership, so nothing happens out of scope
```

s1 will have the same scope as any function parameter, but won't drop the reference. Having references as function parameters borrowing. But you can't modify something that yuo're borrowing. . We hae to explicitly make it mutable using the mut ```&mut s```. But you can only have one mutale reference to a piece of data in a particular scope. This is how Rust precents data races at compile time. We also cannot have a mutable reference while we have an immutable one. But we can have multiple immutable references since those are read only.

Dangling References
---
Dangling references will never occur, the compiler will ensure data will not go out of scope before the reference to the data does.

``` Rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
```

creates the message:

```
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
  | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
  ```

To fix this, we'd have to return the String directly since that will move ownership out.


Slices
---

Slices are a reference to contiguous sequences of elements in a collection. To solve the problem of values changing, we have a thing called a string slice. We can use a string slice to return a reference to part of a string.

Slice syntax - ```&s[0..i]```. So we get a reference to a string s from 0 to i, exclusive.

Slices can be extended to the general case, such as an array. It sotres a refernece to the first element and the length.
