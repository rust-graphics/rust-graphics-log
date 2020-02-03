pub extern crate web_sys;

pub use self::web_sys::console::{error_1, info_1};

pub fn get_now() -> &'static str {
    "thread-id"
}

#[macro_export]
macro_rules! log_i {
    ($fmt:expr) => {
        $crate::info_1(&$crate::log_format!("info", $fmt).into());
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::info_1(&$crate::log_format!("info", $fmt, $($arg)*).into());
    };
}

#[macro_export]
macro_rules! log_e {
    ($fmt:expr) => {
        $crate::error_1(&$crate::log_format!("error", $fmt).into());
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error_1(&$crate::log_format!("error", $fmt, $($arg)*).into());
    };
}

#[macro_export]
macro_rules! log_f {
    ($fmt:expr) => (
        $crate::error_1(&$crate::log_format!("fatal", $fmt).into());
        panic!("");
    );
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error_1(&$crate::log_format!("fatal", $fmt, $($arg)*).into());
        panic!("");
    };
}
