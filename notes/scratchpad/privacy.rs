mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function(); // errors since this is a private fuction in module
    outermost::inside::inner_function(); // errors since accessing public fn outside of outermost module
    outermost::inside::secret_function(); // was never going to work
}
