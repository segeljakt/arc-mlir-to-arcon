//! This file contains an operator which sends and receives ADTs.

mod box_data;
mod box_from_rc;
mod rc_data;
mod rc_from_box;

use rc_data::*;

use arcon::prelude::*;
use arcon_state::Backend;
use kompact::prelude::ComponentDefinition;
use std::rc::Rc;

pub(crate) struct MyOperator {}

impl Operator for MyOperator {
    type IN = box_data::MyData;
    type OUT = i32;
    type TimerState = ArconNever;
    type OperatorState = ();

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> OperatorResult<()> {
        let ArconElement { data, timestamp } = element;

        // Convert from Box<T> into Rc<T>
        let data: MyData = data.into();
        // Other way around, just to see that it works
        let _data: box_data::MyData = (&data).into();

        let data = fold(data.list, 0, |acc, elem| elem + acc);

        ctx.output(ArconElement { data, timestamp });

        Ok(())
    }

    arcon::ignore_timeout!();
    arcon::ignore_persist!();
}

#[tailcall::tailcall]
fn fold<T>(list: Rc<List>, acc: T, agg: impl Fn(T, i32) -> T) -> T {
    if let List::Cons(x) = list.as_ref() {
        let acc = agg(acc, x.val.clone());
        fold(x.tail.clone(), acc, agg)
    } else {
        acc
    }
}
