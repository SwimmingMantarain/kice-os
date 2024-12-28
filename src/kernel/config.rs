pub static mut DEBUG_OUTPUT: bool = false;

#[macro_export]
macro_rules! debug_println {
    ($color:expr, $bg:expr, $($arg:tt)*) => ({
        if unsafe { $crate::kernel::config::DEBUG_OUTPUT } {
            $crate::println!($color, $bg, $($arg)*);
        }
    });
}

#[macro_export]
macro_rules! debug_print {
    ($color:expr, $bg:expr, $($arg:tt)*) => ({
        if unsafe { $crate::kernel::config::DEBUG_OUTPUT } {
            $crate::print!($color, $bg, $($arg)*);
        }
    });
}
