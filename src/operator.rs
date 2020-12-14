// Copyright (c) 2020, KTH Royal Institute of Technology.
// SPDX-License-Identifier: AGPL-3.0-only

//! Example Arc-Script code which should be translated into
//! something which resembles this file. NB: `--` are Arc-Script comments.
//!
//! ```txt
//! type MyData = { i:i32, f:f32 };
//!
//! task MyOperator() (In(MyData)) -> (Out(MyData))
//!
//!     -- State types supported by Arcon.
//!     state state1: Value<MyData>;
//!     state state2: Appender<MyData>;
//!     state state3: Map<u64, MyData>;
//!
//!     on In(data) => {
//!         if let Some(_) = state1.get() {
//!             let foo = { i=0, f=1.1 };
//!             emit Out(foo);
//!             state1.set(foo);
//!             state2.clear();
//!         } else {
//!             state2.append(data);
//!             state3.put(5, data);
//!             emit Out(data);
//!         }
//!     }
//! end
//! ```

use arcon::prelude::state;
use arcon::prelude::*;
use arcon_macros::Arcon;
use arcon_state::{index::IndexOps, Backend};
use kompact::prelude::ComponentDefinition;

#[derive(ArconState)]
pub(crate) struct MyOperator {
    state1: state::Value<MyData, Sled>,
    state2: state::Appender<MyData, Sled>,
    state3: state::Map<u64, MyData, Sled>,
}

type MyInputData = MyData;
type MyOutputData = MyData;

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, prost::Message, Copy, Clone, abomonation_derive::Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(int32, tag = "1")]
    pub i: i32,
    #[prost(float, tag = "2")]
    pub f: f32,
}

impl Operator for MyOperator {
    type IN = MyInputData;
    type OUT = MyOutputData;
    type TimerState = ArconNever;
    type OperatorState = Self;

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> ArconResult<()> {
        // Setup code
        let ArconElement { timestamp, data } = element;

        // Handler generated by arc-mlir, could maybe be inlined if we want.
        my_handler(data, timestamp, self, ctx)?;

        Ok(())
    }

    arcon::ignore_timeout!();

    fn persist(&mut self) -> Result<(), arcon_state::error::ArconStateError> {
        <Self as IndexOps>::persist(self)
    }
}

fn my_handler(
    input_data: MyData,
    input_timestamp: Option<u64>,
    op: &mut MyOperator,
    mut ctx: OperatorContext<MyOperator, impl Backend, impl ComponentDefinition>,
) -> ArconResult<()> {
    let x0 = op.state1.get();
    let x1 = x0.is_some();
    let _x11 = if x1 {
        let x2 = 0;
        let x3 = 1.1;
        let foo = MyData { i: x2, f: x3 };
        let _x4 = ctx.output(ArconElement {
            data: foo,
            timestamp: input_timestamp,
        });
        let _x5 = op.state1.put(foo);
        let x6 = op.state2.clear()?;
        x6
    } else {
        let x7 = 5;
        let _x8 = op.state2.append(input_data)?;
        let _x9 = op.state3.put(x7, input_data)?;
        let x10 = ctx.output(ArconElement {
            data: input_data,
            timestamp: input_timestamp,
        });
        x10
    };
    Ok(())
}
