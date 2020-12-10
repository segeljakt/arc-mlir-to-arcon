// Copyright (c) 2020, KTH Royal Institute of Technology.
// SPDX-License-Identifier: AGPL-3.0-only

use arcon::prelude::state;
use arcon::prelude::*;
use arcon_macros::Arcon;
use arcon_state::{index::IndexOps, Backend};
use kompact::prelude::ComponentDefinition;

/// Example Arc-Script code which should be translated into
/// something which resembles this file. NB: `--` are Arc-Script comments.
///
/// ```txt
/// type MyData = { i:i32, f:f32 };
///
/// task MyOperator() (In(MyData)) -> (Out(MyData))
///
///     -- State types supported by Arcon.
///     state state1: Value<MyData>;
///     state state2: Appender<MyData>;
///     state state3: Map<u64, MyData>;
///
///     on In(data) => {
///         if let Some(_) = state1.get() {
///             let foo = { i=0, f=1.1 };
///             emit Out(foo);
///             state1.set(foo);
///             state2.clear();
///         } else {
///             state2.append(data);
///             state3.put(5, data);
///             emit Out(data);
///         }
///     }
/// end
/// ```

struct MyOperator<B: Backend> {
    state: MyOperatorState<B>,
    outputs: Vec<MyOutputData>,
}

type MyInputData = MyData;
type MyOutputData = MyData;

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, prost::Message, Copy, Clone, abomonation_derive::Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
struct MyData {
    #[prost(int32, tag = "1")]
    pub i: i32,
    #[prost(float, tag = "2")]
    pub f: f32,
}

#[derive(ArconState)]
struct MyOperatorState<B: Backend> {
    state1: state::Value<MyData, B>,
    state2: state::Appender<MyData, B>,
    state3: state::Map<u64, MyData, B>,
}

impl<B: Backend> Operator for MyOperator<B> {
    type IN = MyInputData;
    type OUT = MyOutputData;
    type TimerState = ArconNever;
    type OperatorState = MyOperatorState<B>;

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> ArconResult<()> {
        let timestamp = element.timestamp;

        // No need to use the return values
        let _op = my_handler(element.data, self)?;

        // Clone can be avoided with std::mem::swap
        for data in self.outputs.clone().into_iter() {
            ctx.output(ArconElement { data, timestamp });
        }

        Ok(())
    }

    arcon::ignore_timeout!();

    fn persist(&mut self) -> Result<(), arcon_state::error::ArconStateError> {
        self.state.persist()
    }
}

fn my_handler<B: Backend>(input: MyData, op: &mut MyOperator<B>) -> ArconResult<()> {
    let x0 = op.state.state1.get();
    let x1 = x0.is_some();
    let _x11 = if x1 {
        let x2 = 0;
        let x3 = 1.1;
        let foo = MyData { i: x2, f: x3 };
        let _x4 = op.outputs.push(foo);
        let _x5 = op.state.state1.put(foo);
        let x6 = op.state.state2.clear()?;
        x6
    } else {
        let x7 = 5;
        let _x8 = op.state.state2.append(input)?;
        let _x9 = op.state.state3.put(x7, input)?;
        let x10 = op.outputs.push(input);
        x10
    };
    Ok(())
}
