#[macro_export]
macro_rules! compute_mask {
    (0) => {
        1
    };
    (1) => {
        1
    };
    (_) => {
        0
    };
}

#[macro_export]
macro_rules! compute_val {
    (0) => {
        0
    };
    (1) => {
        1
    };
    (_) => {
        0
    };
}

#[macro_export]
macro_rules! bit_match(
    ($x: expr, ($($b: tt),*)) => ({
        let mut mask = 0;
        let mut val = 0;
        $(
            mask = (mask << 1) | compute_mask!($b);
            val = (val << 1) | compute_val!($b);
        )*
        ($x & mask) == val
    });
);
