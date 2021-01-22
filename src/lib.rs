#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(allocator_api)]

extern crate enum_methods;

mod operators {
    mod arc;
    mod adt;
    mod arena;
    mod box_to_rc;
    mod cache;
    mod immutable;
    mod python;
}

mod constructs {
    mod closure;
}

mod tests {
    #[cfg(test)]
    mod wordcount;
}
