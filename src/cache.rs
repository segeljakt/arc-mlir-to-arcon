//! This file contains an operator which uses caching.

use arcon::prelude::*;
use arcon_macros::Arcon;
use arcon_state::Backend;
use kompact::prelude::ComponentDefinition;

pub(crate) struct MyOperator {}

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, prost::Message, Copy, Clone, abomonation_derive::Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(uint64, tag = "1")]
    pub u: u64,
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
        let ArconElement { data, timestamp } = element;

        let u = fib(data.u);

        ctx.output(ArconElement {
            data: MyData { u },
            timestamp,
        });

        Ok(())
    }

    arcon::ignore_timeout!();
    arcon::ignore_persist!();
}

use cached::proc_macro::cached;

#[cached]
fn fib(n: u64) -> u64 {
    if n > 1 {
        fib(n - 1) + fib(n - 2)
    } else {
        n
    }
}
