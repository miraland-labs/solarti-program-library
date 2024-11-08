macro_rules! miraland_logging {
    ($message:literal, $($arg:tt)*) => {
        #[cfg(feature = "log")]
        ::miraland_program::msg!($message, $($arg)*);
    };
    ($message:literal) => {
        #[cfg(feature = "log")]
        ::miraland_program::msg!($message);
    };
}

macro_rules! log_compute {
    () => {
        #[cfg(all(feature = "mln-log", feature = "log"))]
        ::miraland_program::log::sol_log_compute_units();
    };
}
