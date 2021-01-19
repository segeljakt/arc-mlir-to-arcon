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
    // foo(b: &i32) also works
    fn foo(b: i32) -> impl Fn(i32) -> i32 {
        let f1 = |a: i32| a + b;
        let f2 = |a: i32| a + b;
        let c = std::rc::Rc::new(1);
        let f3 = move |a: i32| a + *c;
        // vec![f1, f2]; ERROR
        bar(f1);
        bar(f2);
        bar(f2);
        bar(f3.clone());
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

mod option_1_1 {
    fn foo(b: std::rc::Rc<i32>) -> impl Fn(i32) -> i32 {
        let f1 = {
            let b = b.clone();
            move |a: i32| a + *b
        };
        let f2 = {
            let b = b.clone();
            move |a: i32| a + *b
        };
        // vec![f1, f2]; ERROR
        bar(f1.clone());
        bar(f2.clone());
        bar(f2.clone());
        f2
    }
    fn bar(f: impl Fn(i32) -> i32) -> i32 {
        f(5)
    }
    // Small addition which explicitly clones all environment variables before
    // closing over them, since some may not implement Copy. Also, Clone the
    // closures before using them or else they cannot be used multiple times.
}

mod option_1_2 {
    fn foo(b: &mut i32) {
        // let f3 = move |a:i32| a + *b;
        // let f3 = move |a:i32| a + *b; // ERROR
        // bar(f3);
        // bar(f3);
        let f1 = |a: i32| a + *b;
        let f2 = |a: i32| a + *b; // OK
        bar(f1);
        bar(f2);
    }
    fn bar(f: impl Fn(i32) -> i32) -> i32 {
        f(5)
    }
    fn baz(a: &mut i32) -> impl Fn(i32, &mut i32) -> i32 + '_ {
        let f1 = move |b: i32, a: &mut i32| {
            *a = b;
            *a
        };
        let f2 = move |b: i32, a: &mut i32| {
            *a = b;
            *a
        };
        qux(f1, f2, a);
        f2
    }
    fn qux(
        f1: impl Fn(i32, &mut i32) -> i32,
        f2: impl Fn(i32, &mut i32) -> i32,
        a: &mut i32,
    ) -> i32 {
        f1(5, a) + f2(5, a)
    }
    // It is not possible to `move` a closure more than once if it contains a
    // mutable environment. Therefore,  If the closure captures a mutable variable (&mut), then the reference cannot
    // be shared if it is moved.
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

/// Option 6: Static dispatch with Rust's Fn trait
mod option_6 {
    fn foo(b: i32) -> impl Fn<(i32,), Output = i32> {
        let f1 = Environment1 { b };
        let f2 = Environment2 { b };
        // vec![f1, f2]; ERROR
        bar(f1);
        // bar(f2); ERROR
        f2
    }
    fn bar(f: impl Fn<(i32,), Output = i32>) -> i32 {
        f(5)
    }
    #[derive(Clone)]
    struct Environment1 {
        b: i32,
    }
    #[derive(Clone)]
    struct Environment2 {
        b: i32,
    }
    impl FnOnce<(i32,)> for Environment1 {
        type Output = i32;
        extern "rust-call" fn call_once(self, (a,): (i32,)) -> Self::Output {
            a + self.b
        }
    }
    impl FnOnce<(i32,)> for Environment2 {
        type Output = i32;
        extern "rust-call" fn call_once(self, (a,): (i32,)) -> Self::Output {
            a * self.b
        }
    }
    impl FnMut<(i32,)> for Environment1 {
        extern "rust-call" fn call_mut(&mut self, (a,): (i32,)) -> Self::Output {
            a + self.b
        }
    }
    impl FnMut<(i32,)> for Environment2 {
        extern "rust-call" fn call_mut(&mut self, (a,): (i32,)) -> Self::Output {
            a * self.b
        }
    }
    impl Fn<(i32,)> for Environment1 {
        extern "rust-call" fn call(&self, (a,): (i32,)) -> Self::Output {
            a + self.b
        }
    }
    impl Fn<(i32,)> for Environment2 {
        extern "rust-call" fn call(&self, (a,): (i32,)) -> Self::Output {
            a * self.b
        }
    }
    // Pros:
    // * Closures automatically become compatible with Rust APIs
    // Cons:
    // * Closure is moved, cannot be reused
}

/// Option 8: &mut as parameter
mod option_7 {
    fn foo(b: &mut i32) -> impl Fn(i32, &mut i32) -> i32 {
        let f1 = move |a: i32, b: &mut i32| {
            *b = a;
            *b
        };
        let f2 = move |a: i32, b: &mut i32| {
            *b = a;
            *b
        };
        bar(f1, b);
        bar(f1, b);
        bar(f2, b);
        f2
    }
    fn bar(mut f: impl Fn(i32, &mut i32) -> i32, b: &mut i32) -> i32 {
        my_foreign_function(|a: i32| f(5, b))
    }
    fn my_foreign_function(mut f: impl FnMut(i32) -> i32) -> i32 {
        f(5)
    }
    // Pros:
    // * Closures automatically become compatible with Rust APIs
    // Cons:
    // * Closure is moved, cannot be reused
}

/// Option 8: Capture Rc<RefCell<State>>
mod option_8 {
    use std::cell::RefCell;
    use std::rc::Rc;
    fn foo(b: Rc<i32>, c: Rc<RefCell<i32>>) -> impl Fn(i32) -> i32 {
        let f1 = {
            let b = b.clone();
            let c = c.clone();
            move |a: i32| {
                {
                    *c.borrow_mut() = a + *b;
                }
                {
                    *c.borrow()
                }
            }
        };
        bar(f1.clone());
        bar(f1.clone());
        f1
    }
    fn bar(f: impl Fn(i32) -> i32) -> i32 {
        f(5)
    }
}

/// Option 9: FnMutant
mod option_9 {

    use fnmutant::{mutant, FnMutant};
    use std::cell::RefCell;
    use std::rc::Rc;
    fn foo(b: &mut i32) -> FnMutant<&mut i32, i32, i32, impl for<'i> Fn(&'i mut i32, i32) -> i32> {
        let f1 = mutant(&|b: &mut i32, a: i32| {
            *b += a;
            *b
        });
        bar(&f1, b);
        bar(&f1, b);
        //         bar(f1, b);
        f1
    }
    fn bar(
        f: &FnMutant<&mut i32, i32, i32, impl for<'i> Fn(&mut i32, i32) -> i32>,
        b: &mut i32,
    ) -> i32 {
        (f.f)(b, 5);
        (f.f)(b, 5)
    }

    fn baz(b: &mut i32, a: i32) -> i32 {
        *b += a;
        *b
    }
}

/// Option 10: Pure functions / "Fibers"
mod option_10 {
    fn foo(b: &mut i32) -> impl Fn(i32, &mut i32) -> i32 {
        move |a: i32, b: &mut i32| {
            *b += a;
            *b
        }
    }
}
