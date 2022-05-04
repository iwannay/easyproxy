#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => (log::error!($($arg)+);exit(1););
    (target: $target:expr, $($arg:tt)+) => (log::error!(target: $target, $(arg)+);exit(1));
}
