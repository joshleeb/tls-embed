macro_rules! arr {
    ($($x:expr),*) => (
        $crate::codec::array::Array::from([$($x),*].as_ref())
    );
    ($($x:expr,)*) => (arr![$($x),*])
}
