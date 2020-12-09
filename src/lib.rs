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
}

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, prost::Message, Copy, Clone, abomonation_derive::Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub struct MyData {
    #[prost(int32, tag = "1")]
    pub i: i32,
    #[prost(float, tag = "2")]
    pub f: f32,
}

type MyStateType1<B> = state::Value<MyData, B>;
type MyStateType2<B> = state::Appender<MyData, B>;
type MyStateType3<B> = state::Map<u64, MyData, B>;

#[derive(ArconState)]
struct MyOperatorState<B: Backend> {
    state1: MyStateType1<B>,
    state2: MyStateType2<B>,
    state3: MyStateType3<B>,
}

impl<B: Backend> Operator for MyOperator<B> {
    type IN = MyData;
    type OUT = MyData;
    type TimerState = ArconNever;
    type OperatorState = MyOperatorState<B>;

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> ArconResult<()> {
        let timestamp = element.timestamp;

        let mut outputs = Vec::new();

        // No need to use the return values
        let (_state1, _state2, _state3, _outputs_x) = my_handler(
            element.data,
            &mut self.state.state1,
            &mut self.state.state2,
            &mut self.state.state3,
            &mut outputs,
        )?;

        for data in outputs {
            ctx.output(ArconElement { data, timestamp });
        }

        Ok(())
    }

    arcon::ignore_timeout!();

    fn persist(&mut self) -> Result<(), arcon_state::error::ArconStateError> {
        self.state.persist()
    }
}

type State<'i, T> = &'i mut T;

macro_rules! functional {
    { $var:ident $($rest:tt)* } => { { $var $($rest)*; $var } }
}

fn my_handler<'i, B: Backend>(
    input: MyData,
    state1: State<'i, MyStateType1<B>>,
    state2: State<'i, MyStateType2<B>>,
    state3: State<'i, MyStateType3<B>>,
    outputs: State<'i, Vec<MyData>>,
) -> ArconResult<(
    State<'i, MyStateType1<B>>,
    State<'i, MyStateType2<B>>,
    State<'i, MyStateType3<B>>,
    State<'i, Vec<MyData>>,
)> {
    let x0 = state1.get();
    let x1 = x0.is_some();
    let (state1_x1, state2_x2, state3_x1, outputs_x3) = if x1 {
        let x2 = 0;
        let x3 = 1.1;
        let foo = MyData { i: x2, f: x3 };
        let outputs_x0 = functional!(outputs.push(foo));
        let state1_x0 = functional!(state1.put(foo));
        let state2_x0 = functional!(state2.clear()?);
        let x4 = (state1_x0, state2_x0, state3, outputs_x0);
        x4
    } else {
        let x5 = 5;
        let state2_x1 = functional!(state2.append(input)?);
        let state3_x0 = functional!(state3.put(x5, input)?);
        let outputs_x1 = functional!(outputs.push(input));
        let x6 = (state1, state2_x1, state3_x0, outputs_x1);
        x6
    };
    let x7 = (state1_x1, state2_x2, state3_x1, outputs_x3);
    let x8 = Ok(x7);
    x8
}
