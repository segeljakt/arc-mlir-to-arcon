//! This file contains an operator which uses immutable data structures.

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

        let u = test(data.u);

        ctx.output(ArconElement {
            data: MyData { u },
            timestamp,
        });

        Ok(())
    }

    arcon::ignore_timeout!();
    arcon::ignore_persist!();
}

fn test(_: u64) -> u64 {
    let _: im_rc::Vector<i32> = im_rc::Vector::new();
    let _: im_rc::HashMap<u32, i32> = im_rc::HashMap::new();
    let _: im_rc::HashSet<i32> = im_rc::HashSet::new();
    let _: im_rc::OrdSet<i32> = im_rc::OrdSet::new();
    let _: im_rc::OrdMap<u32, i32> = im_rc::OrdMap::new();
    let _: rpds::Vector<i32> = rpds::Vector::new();
    let _: rpds::List<i32> = rpds::List::new();
    let _: rpds::Stack<i32> = rpds::Stack::new();
    let _: rpds::Queue<i32> = rpds::Queue::new();
    let _: rpds::HashTrieSet<i32> = rpds::HashTrieSet::new();
    let _: rpds::HashTrieMap<u32, i32> = rpds::HashTrieMap::new();
    let _: rpds::RedBlackTreeSet<i32> = rpds::RedBlackTreeSet::new();
    let _: rpds::RedBlackTreeMap<u32, i32> = rpds::RedBlackTreeMap::new();
    let _: ppar::rc::Vector<i32> = ppar::rc::Vector::new();
    3
}
