pub macro join_many($first:expr, $second:expr) {
    ($first:expr, $second:expr) => {
        join($first, $second)
    };
    ($first:expr, $second:expr, $($rest:expr),*) => {
        join($first, join_many!($second, $($rest),*))
    };
}
