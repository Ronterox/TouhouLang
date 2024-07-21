#[doc(hidden)]
pub const unsafe fn concat<A: Copy, B: Copy, C: Copy>(a: A, b: B) -> C {
    #[repr(C)]
    struct Both<A, B>(A, B);

    union Transmute<A, B, C> {
        from: std::mem::ManuallyDrop<Both<A, B>>,
        to: std::mem::ManuallyDrop<C>,
    }

    std::mem::ManuallyDrop::into_inner(Transmute { from: std::mem::ManuallyDrop::new(Both(a, b)) }.to)
}

#[macro_export]
macro_rules! concat_array {
    () => {
        []
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        let c: [_; $a.len() + $b.len()] = unsafe { $crate::concat(a, b) };
        // Constrain the element types to be the same to guide inference.
        let _: [*const _; 3] = [a.as_ptr(), b.as_ptr(), c.as_ptr()];
        c
    }};
    ($a:expr, $($rest:expr),*) => {
        concat_array!($a, concat_array!($($rest),*))
    };
    ($a:expr, $($rest:expr),*,) => {
        concat_array!($a, $($rest),*)
    };
}

