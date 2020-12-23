//! This file contains an operator which sends and receives ADTs.

use abomonation_derive::Abomonation;
use arcon::prelude::*;
use arcon_macros::Arcon;
use arcon_state::Backend;
use kompact::prelude::ComponentDefinition;
use prost::{Message, Oneof};

pub(crate) struct MyOperator {}

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, Message, Clone, Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(oneof = "List", tags = "1, 2, 3")]
    pub list: Option<List>,
}

#[derive(Oneof, Clone, Abomonation)]
pub(crate) enum List {
    #[prost(message, tag = "1")]
    Cons(Box<Cons>),
    #[prost(message, tag = "2")]
    Nil(Nil),
}

#[derive(Message, Clone, Abomonation)]
pub(crate) struct Cons {
    #[prost(int32, tag = "3")]
    val: i32,
    #[prost(oneof = "List", tags = "1, 2")]
    tail: Option<List>,
}

#[derive(Message, Clone, Abomonation)]
pub(crate) struct Nil {}

impl Operator for MyOperator {
    type IN = MyData;
    type OUT = i32;
    type TimerState = ArconNever;
    type OperatorState = ();

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> ArconResult<()> {
        let ArconElement { data, timestamp } = element;

        let mut list = data.list;
        let mut data = 0;
        while let Some(List::Cons(e)) = list {
            data += e.val;
            list = e.tail
        }

        ctx.output(ArconElement { data, timestamp });

        Ok(())
    }

    arcon::ignore_timeout!();
    arcon::ignore_persist!();
}
