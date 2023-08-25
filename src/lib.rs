/// Same as println! but with red color output
#[macro_export]
macro_rules! println_red {
    ($($arg:tt)*) => ({
        println!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as println! but with green color output
#[macro_export]
macro_rules! println_green {
    ($($arg:tt)*) => ({
        println!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as println! but with yellow color output
#[macro_export]
macro_rules! println_yellow {
    ($($arg:tt)*) => ({
        println!("\x1b[33m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as eprintln! but with red color output
#[macro_export]
macro_rules! eprintln_red {
    ($($arg:tt)*) => ({
        eprintln!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as eprintln! but with green color output
#[macro_export]
macro_rules! eprintln_green {
    ($($arg:tt)*) => ({
        eprintln!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as eprintln! but with yellow color output
#[macro_export]
macro_rules! eprintln_yellow {
    ($($arg:tt)*) => ({
        eprintln!("\x1b[33m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as panic! but with red color output
#[macro_export]
macro_rules! panic_red {
    ($($arg:tt)*) => ({
        panic!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as panic! but with green color output
#[macro_export]
macro_rules! panic_green {
    ($($arg:tt)*) => ({
        panic!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
    });
}

/// Same as panic! but with yellow color output
#[macro_export]
macro_rules! panic_yellow {
    ($($arg:tt)*) => ({
        panic!("\x1b[33m{}\x1b[0m", format_args!($($arg)*));
    });
}
