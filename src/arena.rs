//! This file contains an operator which internally uses an arena allocator
//! for all local allocations.

use arcon::prelude::*;
use arcon_macros::Arcon;
use arcon_state::Backend;
use bumpalo::boxed::Box;
use bumpalo::collections::String;
use bumpalo::collections::Vec;
use kompact::prelude::ComponentDefinition;

pub(crate) struct MyOperator {
    arena: bumpalo::Bump,
}

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, prost::Message, Copy, Clone, abomonation_derive::Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(int32, tag = "1")]
    pub i: i32,
}

impl Operator for MyOperator {
    type IN = MyData;
    type OUT = MyData;
    type TimerState = ArconNever;
    type OperatorState = ();

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> ArconResult<()> {
        // Always begin by resetting the allocator to 0.
        self.arena.reset();

        // Demonstration code (Probably a bit too complicated to generate directly)
        test_list(self, element.clone(), &mut ctx);
        test_vec(self, element.clone(), &mut ctx);
        test_string(self, element.clone(), &mut ctx);

        Ok(())
    }

    arcon::ignore_timeout!();
    arcon::ignore_persist!();
}

enum List<'i, T> {
    Cons(T, Box<'i, List<'i, T>>),
    Nil,
}

/// Tests arena-allocating a custom linked-list
fn test_list(
    op: &mut MyOperator,
    element: ArconElement<MyData>,
    ctx: &mut OperatorContext<MyOperator, impl Backend, impl ComponentDefinition>,
) {
    // Create a big linked list in a single allocation.
    let mut list = List::Nil;
    for i in 0..element.data.i {
        list = List::Cons(i, Box::new_in(list, &op.arena));
    }

    // Fold the thing imperatively while emitting output.
    let mut sum = 0;
    let mut iter = &list;
    while let List::Cons(i, tail) = iter {
        sum += i;
        iter = tail;
        ctx.output(ArconElement {
            timestamp: element.timestamp,
            data: MyData { i: sum },
        });
    }
}

/// Tests arena-allocating a vector
fn test_vec(
    op: &mut MyOperator,
    element: ArconElement<MyData>,
    ctx: &mut OperatorContext<MyOperator, impl Backend, impl ComponentDefinition>,
) {
    // Same code as test_list
    let mut vec = Vec::new_in(&op.arena);
    for i in 0..element.data.i {
        vec.push(i);
    }
    let mut sum = 0;
    while let Some(i) = vec.pop() {
        sum += i;
        ctx.output(ArconElement {
            timestamp: element.timestamp,
            data: MyData { i: sum },
        });
    }
}

/// Tests arena-allocating a string
fn test_string(
    op: &mut MyOperator,
    element: ArconElement<MyData>,
    ctx: &mut OperatorContext<MyOperator, impl Backend, impl ComponentDefinition>,
) {
    let mut string = String::new_in(&op.arena);
    for _ in 0..element.data.i {
        string.push('a');
    }
    while let Some(c) = string.pop() {
        ctx.output(ArconElement {
            timestamp: element.timestamp,
            data: MyData { i: c as i32 },
        });
    }
}
