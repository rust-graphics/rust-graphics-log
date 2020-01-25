#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! log_i {
    ($fmt:expr) => {
        println!("{}", format!("[info] ({}:{}) {}", file!(), line!(), $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!("{}", format!("[info] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! log_e {
    ($fmt:expr) => {
        eprintln!("{}", format!("[error] ({}:{}) {}", file!(), line!(), $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}", format!("[error] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! log_f {
    ($fmt:expr) => (
        panic!("{}", format!("[fatal] ({}:{}) {}", file!(), line!(), $fmt));
    );
    ($fmt:expr, $($arg:tt)*) => {
        panic!("{}", format!("[fatal] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg_attr(target_os = "android", link(name = "log", kind = "dylib"))]
extern "C" {
    fn __android_log_write(
        priority: std::os::raw::c_int,
        tag: *const std::os::raw::c_char,
        text: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
}

#[cfg(target_os = "android")]
fn print(priority: i32, text: &str) {
    use std::{ffi::CString, os::raw::c_int};
    let tag = CString::new("rust-graphics-log").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        __android_log_write(priority as c_int, tag.as_ptr(), text.as_ptr());
    }
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! log_i {
    ($fmt:expr) => {
        $crate::print(4, &format!("[info] ({}:{}) {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print(4, &format!("[info] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! log_e {
    ($fmt:expr) => {
        $crate::print(6, &format!("[error] ({}:{}) {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print(6, &format!("[error] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! log_f {
    ($fmt:expr) => ({
        $crate::print(7, &format!("[fatal] ({}:{}) {}", file!(), line!(), format!($fmt)));
        panic!("{}", &format!("[fatal] ({}:{}) {}", file!(), line!(), format!($fmt)));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::print(7, &format!("[fatal] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
        panic!("{}", &format!("[fatal] ({}:{}) {}", file!(), line!(), format!($fmt, $($arg)*)));
    });
}

#[macro_export]
macro_rules! unwrap_f {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => log_f!("Unwrap failed!"),
        }
    };
}

#[macro_export]
macro_rules! result_f {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => log_f!("Unwrap failed! {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! unimplemented_f {
    () => {
        log_f!("Not implemented")
    };
}

#[macro_export]
macro_rules! unexpected_f {
    () => {
        log_f!("Unexpected")
    };
}

#[macro_export]
macro_rules! todo_e {
    () => {
        log_e!("TODO")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_1() {
        log_i!("Test 1");
        log_i!("Test {}", 2);
        log_e!("Test 3");
        log_e!("Test {}", 4);
    }

    #[test]
    #[should_panic]
    fn this_test_2() {
        log_f!("Test 5");
    }

    #[test]
    #[should_panic]
    fn this_test_3() {
        log_f!("Test 6");
    }
}
