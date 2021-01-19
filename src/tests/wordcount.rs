use arcon::prelude::*;
use std::sync::Arc;

pub mod arc_types {
    use arcorn::arcorn;
    // Generates a `mod arcon_types { ... }`
    //
    // Arc-structs/enums are currently allowed to contain:
    // * Nominals: Rc<MyStruct>, Rc<MyEnum>
    // * Primitives: String, u32, u64, i32, i64
    use std::rc::Rc;
    arcorn! {
        pub struct Input {
            pub key: Rc<String>,
        }

        pub struct Output {
            pub key: Rc<String>,
            pub val: u64,
        }
    }
}

pub mod arc_operators {
    use super::arc_types;
    use super::arc_types::arcon_types;
    use arcon::prelude::state::backend::Sled;
    use arcon::prelude::state::index::IndexOps;
    use arcon::prelude::state::ArconState;
    use arcon::prelude::state::Backend;
    use arcon::prelude::*;
    use arcorn::state::ArcMap;
    use std::rc::Rc;
    #[derive(ArconState)]
    pub struct MapReduce {
        pub state: ArcMap<String, u64, Sled>,
    }
    impl Operator for MapReduce {
        type IN = arcon_types::Input;
        type OUT = arcon_types::Output;
        type TimerState = ArconNever;
        type OperatorState = Self;

        fn handle_element(
            &mut self,
            element: ArconElement<Self::IN>,
            mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
        ) -> OperatorResult<()> {
            let ArconElement { data, timestamp } = element;

            // Arcon => Arc Deserialisation
            let arcon_input: arcon_types::Input = data;
            let arc_input: Rc<arc_types::Input> = Rc::new(arcon_input.into()); // TODO: Make into() return Rc

            // Operator logic

            // TODO: Can we avoid conversions with state-access?
            let arcon_key = arc_input.key.as_ref().into();
            if self.state.contains(arcon_key)? {
                let arcon_key = arc_input.key.as_ref().into();
                let val = self.state.get_unchecked(arcon_key)?;

                let arcon_key = arc_input.key.as_ref().into();
                self.state.insert(arcon_key, val + 1)?;
            } else {
                let arcon_key = arc_input.key.as_ref().into();
                self.state.insert(arcon_key, 1)?;
            }

            let arcon_key = arc_input.key.as_ref().into();
            let arc_val = self.state.get_unchecked(arcon_key)?;

            // Arc => Arcon Serialisation
            let arc_output: Rc<arc_types::Output> = Rc::new(arc_types::Output {
                val: arc_val,
                key: arc_input.key.clone(),
            });
            let arcon_output: arcon_types::Output = arc_output.as_ref().into();

            let data = arcon_output;
            let element = ArconElement { data, timestamp };
            ctx.output(element);
            Ok(())
        }

        arcon::ignore_timeout!();

        fn persist(&mut self) -> Result<(), arcon_state::error::ArconStateError> {
            <Self as IndexOps>::persist(self)
        }
    }
}

#[test]
fn wordcount() {
    // Generated arcon-compatible types
    use arc_types::arcon_types::Input;
    use arcorn::state::ArcMap;

    let words = include_str!("./hamlet.txt")
        .split_whitespace()
        .map(ToString::to_string)
        .map(|key| Input { key })
        .collect::<Vec<Input>>();

    let mut pipeline = Pipeline::default()
        .collection(words, |conf| {
            conf.set_arcon_time(ArconTime::Process);
        })
        .operator(OperatorBuilder {
            constructor: Arc::new(|b| arc_operators::MapReduce {
                state: ArcMap::new("my_state", b).unwrap(),
            }),
            conf: Default::default(),
        })
        // TODO: KeyBy((key, val) -> key)
        // TODO: Reduce((val_a, val_b) -> val_a + val_b)
        .to_console()
        .build();

    pipeline.start();
    pipeline.await_termination();
}
