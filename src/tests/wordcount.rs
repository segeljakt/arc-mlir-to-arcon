use arcon::prelude::*;
use std::sync::Arc;

mod arc_types {
    use arcorn::arcorn;
    // Generates a `mod arcon_types { ... }`
    //
    // Arc-structs/enums are currently allowed to contain:
    // * Nominals: Rc<MyStruct>, Rc<MyEnum>
    // * Primitives: String, u32, u64, i32, i64
    arcorn! {
        pub struct Data0 {
            pub key: String,
        }

        pub struct Data1 {
            pub key: String,
            pub val: u64,
        }
    }
}

#[test]
fn wordcount() {
    // Generated arcon-compatible types
    use arc_types::arcon_types::{Data0, Data1};

    let words = include_str!("./hamlet.txt")
        .split_whitespace()
        .map(ToString::to_string)
        .map(|key| Data0 { key })
        .collect::<Vec<Data0>>();

    let mut pipeline = Pipeline::default()
        .collection(words, |conf| {
            conf.set_arcon_time(ArconTime::Process);
        })
        // Map(key -> (key, 0))
        .operator(OperatorBuilder {
            constructor: Arc::new(|_| Map::new(|Data0 { key }: Data0| Data1 { val: 0, key })),
            conf: Default::default(),
        })
        // TODO: GroupBy((key, val) -> key)
        // TODO: Reduce((val_a, val_b) -> val_a + val_b)
        .to_console()
        .build();

    pipeline.start();
    pipeline.await_termination();
}
