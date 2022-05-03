//
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => (log::error!($($arg)+);exit(1););
    (target: $target:expr, $($arg:tt)+) => (log::error!(target: $target, $(arg)+);exit(1));
}

// macro_rules! error {
//     // error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
//     // error!(target: "my_target", "a {} event", "log")
//     (target: $target:expr, $($arg:tt)+) => (log!(target: $target, $crate::Level::Error, $($arg)+));

//     // error!("a {} event", "log")
//     ($($arg:tt)+) => (log!($crate::Level::Error, $($arg)+))
// }
