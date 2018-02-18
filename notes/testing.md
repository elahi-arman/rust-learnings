# Test Functions

A test function is just one that's annotated with a test attribute. Just add #[test] before the definition. Running cargo test builds a test runner binary.

We could also have non-test functions in the tests module to help set common scenarios.

Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

``` rust
 // Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this tests module.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

# Assertions

The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true. We give the assert! macro an argument that evaluates to a Boolean. If the value is true, assert! does nothing and the test passes. If the value is false, the assert! macro calls the panic! macro, which causes the test to fail

For testing equality and non-equality -- the values being compared must implement the PartialEq and Debug traits. this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition

Anything passed into the assert after the required parameters will be passed into the format! macro.

Add ```#[should_panic]``` to a test to make sure that it panics. Pass an expected value to verify the panic message panics for the proper reason.

- assert!(expr)-- tests true
- assert!(!expr) -- tests false
- assert_eq!(x, y) -- tests equality
- assert_nq!(x, y) -- tests non-equality

# Running tests

Because the tests are running at the same time, make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.

`cargo test -- --nocapture` -- to get the value of successful functions
`cargo test add` -- only tests functions with add in the name

Add the `#[ignore]` macro to a test to ignore any tests. And use `cargo test -- --ignored to only run those tests.`

# Test Organization

## Unit tests
Put unit tests in the src directory in each file with the code they're testing, Create a tests module and annotate the module with cfg(tests). , Rust’s privacy rules do allow you to test private functions.

## Integration tests
Create a tests directory at the top level of thr project directory (next to src)
