#![feature(thread_id_value)]

pub fn get_thread_id() -> u64 {
    return std::thread::current().id().as_u64();
}

#[macro_export]
macro_rules! log_format {
    ($t:expr, $fmt:expr) => {
        format!("<{}:{}> [{}] ({}:{}) {}",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() - 1580022711607150410,
            $crate::get_thread_id(),
            $t, file!(), line!(), $fmt)
    };
    ($t:expr, $fmt:expr, $($arg:tt)*) => {
        format!("<{}:{}> [{}] ({}:{}) {}",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() - 1580022711607150410,
            $crate::get_thread_id(),
            $t, file!(), line!(), format!($fmt, $($arg)*))
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! log_i {
    ($fmt:expr) => {
        println!("{}", $crate::log_format!("info", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!("{}", $crate::log_format!("info", $fmt, $($arg)*));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! log_e {
    ($fmt:expr) => {
        eprintln!("{}", $crate::log_format!("error", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}", $crate::log_format!("error", $fmt, $($arg)*));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! log_f {
    ($fmt:expr) => (
        panic!("{}", $crate::log_format!("fatal", $fmt));
    );
    ($fmt:expr, $($arg:tt)*) => {
        panic!("{}", $crate::log_format!("fatal", $fmt, $($arg)*));
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
pub fn print(priority: i32, text: &str) {
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
        $crate::print(4, &$crate::log_format!("info", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print(4, &$crate::log_format!("info", $fmt, $($arg)*));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! log_e {
    ($fmt:expr) => {
        $crate::print(6, &$crate::format!("error", $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print(6, &$crate::format!("error", $fmt, $($arg)*));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! log_f {
    ($fmt:expr) => ({
        let msg = format!("fatal", $fmt);
        $crate::print(7, &msg);
        panic!("{}", &msg);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        let msg = format!("fatal", $fmt, $($arg)*);
        $crate::print(7, &msg);
        panic!("{}", &msg);
    });
}

#[macro_export]
macro_rules! unwrap_f {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => $crate::log_f!("Unwrap failed!"),
        }
    };
}

#[macro_export]
macro_rules! result_f {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => $crate::log_f!("Unwrap failed! {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! unimplemented_f {
    () => {
        $crate::log_f!("Not implemented")
    };
}

#[macro_export]
macro_rules! unexpected_f {
    () => {
        $crate::log_f!("Unexpected")
    };
}

#[macro_export]
macro_rules! todo_e {
    () => {
        $crate::log_e!("TODO")
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
        todo_e!();
    }

    #[test]
    #[should_panic]
    fn this_test_2() {
        log_f!("Test 5");
    }

    #[test]
    #[should_panic]
    fn this_test_3() {
        unimplemented_f!();
    }

    #[test]
    #[should_panic]
    fn this_test_4() {
        unexpected_f!();
    }
}
