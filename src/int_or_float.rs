// Trait exposing all (most?) of the shared behavior of the `i*`, `u*` and `f*` types
// Surprisingly, no such trait seems to exist in the Rust ecosystem.
pub trait IntOrFloat:
    num_traits::NumAssign
    + std::cmp::PartialOrd
    + std::marker::Copy
    + std::fmt::Display
    + std::fmt::Debug
{
    // Truly largest value of the given type.
    // (`T::MAX` gives the largest *finite* value, but the largest value for float types is `T::INFINITY`).
    // Again quite surprising that no such trait seems to exist in the Rust ecosystem.
    fn max() -> Self;
}

// Helper macros to implement `IntOrFloat::max`
macro_rules! impl_IntOrFloat_max_via_MAX {
    ($T:ident) => {
        impl IntOrFloat for $T {
            fn max() -> $T { $T::MAX }
        }
    }
}
macro_rules! impl_IntOrFloat_max_via_INFINITY {
    ($T:ident) => {
        impl IntOrFloat for $T {
            fn max() -> $T { $T::INFINITY }
        }
    }
}

// `IntOrFloat` implementations
impl_IntOrFloat_max_via_MAX!(usize);
impl_IntOrFloat_max_via_MAX!(u8);
impl_IntOrFloat_max_via_MAX!(u16);
impl_IntOrFloat_max_via_MAX!(u32);
impl_IntOrFloat_max_via_MAX!(u64);
impl_IntOrFloat_max_via_MAX!(i8);
impl_IntOrFloat_max_via_MAX!(i16);
impl_IntOrFloat_max_via_MAX!(i32);
impl_IntOrFloat_max_via_MAX!(i64);
impl_IntOrFloat_max_via_INFINITY!(f32);
impl_IntOrFloat_max_via_INFINITY!(f64);