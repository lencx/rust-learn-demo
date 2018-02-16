/*
mod outermost {
    pub fn middle_fn() {}

    fn middle_secret_fn() {
        ::outermost::inside::inner_fn();
        // function `secret_fn` is private
        // ::outermost::inside::secret_fn(); // error
    }

    mod inside {
        pub fn inner_fn() {
            ::outermost::middle_fn(); // ok
            ::outermost::middle_secret_fn(); // ok
        }

        fn secret_fn() {
            ::outermost::middle_secret_fn(); // ok
            ::outermost::inside::inner_fn(); // ok
        }
    }
}

fn try_me() {
    // outermost::middle_fn(); // ok
    // function `middle_secret_fn` is private
    // outermost::middle_secret_fn(); // error

    // module `inside` is private
    // outermost::inside::inner_fn(); // error
    // outermost::inside::secret_fn(); // error
}
*/

mod outermost {
    pub fn middle_fn() {}

    fn middle_secret_fn() {
        ::outermost::inside::inner_fn();
        // function `secret_fn` is private
        ::outermost::inside::secret_fn(); // error
    }

    pub mod inside {
        pub fn inner_fn() {
            ::outermost::middle_fn(); // ok
            ::outermost::middle_secret_fn(); // ok
        }

        fn secret_fn() {
            ::outermost::middle_secret_fn(); // ok
            ::outermost::inside::inner_fn(); // ok
        }
    }
}

fn try_me() {
    outermost::middle_fn(); // ok
    // function `middle_secret_fn` is private
    // outermost::middle_secret_fn(); // error

    outermost::inside::inner_fn(); // ok
    // function `secret_fn` is private
    // outermost::inside::secret_fn(); // error
}