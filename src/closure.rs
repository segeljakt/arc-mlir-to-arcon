#![allow(unused)]

//! This file contains some experiments for how to translate closures into Rust.
//!
//! Arc-script code:
//! ```txt
//! fun foo(b: i32, c: i32)
//!     let f1 = |a: i32| a + b;
//!     let f2 = |a: i32| a + c;
//!     bar(f1);
//!     bar(f2);
//! end
//!
//! fun bar(f: i32 -> i32)
//!     f(5)
//! end
//! ```

/// Option 1: Static dispatch
mod option_1 {
    fn foo(b: i32) -> impl Fn(i32) -> i32 {
        let f1 = |a: i32| a + b;
        let f2 = |a: i32| a + b;
        let f3 = move |a: i32| a + b;
        // vec![f1, f2]; ERROR
        bar(f1);
        bar(f2);
        bar(f2);
        // f2 ERROR
        f3
    }
    fn bar(f: impl Fn(i32) -> i32) -> i32 {
        f(5)
    }
    // Pros:
    // * Maximum performance
    // Cons:
    // * Cannot store multiple closures in the same vector
    // * Must move environment when moving out of scope
}

/// Option 2: Dynamic dispatch
mod option_2 {
    use std::rc::Rc;
    fn foo(b: i32) -> Rc<dyn Fn(i32) -> i32> {
        let f1: Rc<dyn Fn(i32) -> i32> = Rc::new(move |a: i32| a + b);
        let f2: Rc<dyn Fn(i32) -> i32> = Rc::new(move |a: i32| a + b);
        bar(f1.clone());
        bar(f2.clone());
        vec![f1.clone(), f2.clone()];
        f2.clone()
    }
    fn bar(f: Rc<dyn Fn(i32) -> i32>) -> i32 {
        f(5)
    }
    // Pros:
    // * Can store multiple closures in the same vector
    // Cons:
    // * Worse performance
    // * Must always move environment
}

/// Option 3: Static Dispatch with custom Closure trait
mod option_3 {
    use std::rc::Rc;
    fn foo(b: i32) -> impl Closure<I = (i32,), O = i32> {
        let f1 = Environment1 { b };
        let f2 = Environment2 { b };
        // vec![f1, f2]; ERROR
        bar(f1);
        // bar(f2); ERROR
        f2
    }
    fn bar(f: impl Closure<I = (i32,), O = i32>) -> i32 {
        f.call((5,))
    }
    trait Closure {
        type I;
        type O;
        fn call(&self, args: Self::I) -> Self::O;
    }
    #[derive(Clone)]
    struct Environment1 {
        b: i32,
    }
    #[derive(Clone)]
    struct Environment2 {
        b: i32,
    }
    impl Closure for Environment1 {
        type I = (i32,);
        type O = i32;
        fn call(&self, (a,): Self::I) -> Self::O {
            a + self.b
        }
    }
    impl Closure for Environment2 {
        type I = (i32,);
        type O = i32;
        fn call(&self, (a,): Self::I) -> Self::O {
            a * self.b
        }
    }
    // Pros:
    // * More control over how environment is moved
    // Cons:
    // * Closure is moved, cannot be reused
}

/// Option 4: Dynamic Dispatch with Custom Closure trait
mod option_4 {
    use std::rc::Rc;
    fn foo(b: i32) -> Rc<dyn Closure<I = (i32,), O = i32>> {
        let f1: Rc<dyn Closure<I = (i32,), O = i32>> = Rc::new(Environment1 { b });
        let f2: Rc<dyn Closure<I = (i32,), O = i32>> = Rc::new(Environment2 { b });
        vec![f1.clone(), f2.clone()];
        bar(f1.clone());
        bar(f2.clone());
        bar(f2.clone());
        f2
    }
    fn bar(f: Rc<dyn Closure<I = (i32,), O = i32>>) -> i32 {
        f.call((5,))
    }
    trait Closure {
        type I;
        type O;
        fn call(&self, args: Self::I) -> Self::O;
    }
    struct Environment1 {
        b: i32,
    }
    struct Environment2 {
        b: i32,
    }
    impl Closure for Environment1 {
        type I = (i32,);
        type O = i32;
        fn call(&self, (a,): Self::I) -> Self::O {
            a + self.b
        }
    }
    impl Closure for Environment2 {
        type I = (i32,);
        type O = i32;
        fn call(&self, (a,): Self::I) -> Self::O {
            a * self.b
        }
    }
    // Same as previous, except:
    // Pros:
    // * More control over how environment is moved
    // * Can store closures in vectors
    // Cons:
    // * Performance loss
}

/// Option 5: Static dispatch with function pointers
mod option_5 {
    use std::rc::Rc;
    fn foo(b: i32) -> Rc<Closure1> {
        let f1: Rc<Closure1> = Rc::new(Closure1 { fun: f1, env: (b,) });
        let f2: Rc<Closure2> = Rc::new(Closure2 { fun: f2, env: (b,) });
//         vec![f1.clone(), f2.clone()]; NOT OK
        bar_f1(f1.clone());
        bar_f1(f1.clone());
        bar_f2(f2.clone());
        f1
    }
    fn bar_f1(f: Rc<Closure1>) -> i32 {
        (f.fun)(5, f.env)
    }
    fn bar_f2(f: Rc<Closure2>) -> i32 {
        (f.fun)(5, f.env)
    }
    fn f1(a: i32, (b,): (i32,)) -> i32 {
        a + b
    }
    fn f2(a: i32, (b,): (i32,)) -> i32 {
        a * b
    }
    struct Closure1 {
        fun: fn(i32, (i32,)) -> i32,
        env: (i32,),
    }
    struct Closure2 {
        fun: fn(i32, (i32,)) -> i32,
        env: (i32,),
    }
    // Same as previous, except:
    // Pros:
    // * Lower-level (no traits involved)
    // Cons:
    // * Cannot store closures in vectors
    // * Must monomorphise everything
}
