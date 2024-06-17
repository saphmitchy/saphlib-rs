use std::fmt::Display;

/// output args with separation of a space to stdout.
/// Newline is inserted in the end.
/// ```
/// use misc::out_with_space;
///
/// out_with_space!(); // prints just a newline
/// out_with_space!("foo"); // prints `foo`
/// out_with_space!("foo", 42); // prints `foo 42`
/// ```
#[macro_export]
macro_rules! out_with_space {
    ($ ( $args:expr ), *) => {
        out_with_space!(@init $($args),*);
        print!("\n");
    };
    (@init $arg:expr, $ ( $args:expr ) , *) => {
        print!("{}", $arg);
        $(
            print!(" {}", $args);
        )*
    };
    (@init $arg:expr) => {
        print!("{}", $arg);
    };
    (@init) => {};
}

/// output args with separation of a newline to stdout.
/// Newline is inserted in the end.
/// ```
/// use misc::out_with_nl;
///
/// out_with_nl!(); // prints just a newline
/// out_with_nl!("foo"); // prints `foo\n`
/// out_with_nl!("foo", 42); // prints `foo\n42\n`
/// ```
#[macro_export]
macro_rules! out_with_nl {
    ($ ( $args:expr ), *) => {
        $(
            println!("{}", $args);
        )*
    };
}

pub trait Joinable {
    /// stringify items with separation of a space.
    /// ```
    /// use misc::Joinable;
    ///
    /// let mut a = vec![3, 1, 4];
    /// assert_eq!(a.iter().join_sep(), "3 1 4");
    /// ```
    fn join_sep(&mut self) -> String;
    /// output items with separation of a space to stdout.
    /// ```
    /// use misc::Joinable;
    ///
    /// let mut a = vec![3, 1, 4];
    /// a.iter().output();
    /// ```
    fn output(&mut self) -> ();
}

impl<T> Joinable for T
where
    T: Iterator,
    T::Item: Display,
{
    fn join_sep(&mut self) -> String {
        match self.next() {
            None => String::new(),
            Some(first) => {
                let mut res = first.to_string();
                for elem in self {
                    // write!(&mut res, " {}", elem);
                    res.push_str(format!(" {}", elem).as_str());
                }
                res
            }
        }
    }

    fn output(&mut self) -> () {
        println!("{}", self.join_sep());
    }
}
