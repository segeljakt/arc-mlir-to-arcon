use abomonation_derive::Abomonation;
use arcon_macros::Arcon;
use prost::{Message, Oneof};

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, Message, Clone, Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(oneof = "List", tags = "1, 2")]
    pub(crate) list: Option<List>,
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
    #[prost(int32, repeated, tag = "3")]
    pub(crate) val: Vec<i32>,
    #[prost(oneof = "List", tags = "1, 2")]
    pub(crate) tail: Option<List>,
}

#[derive(Message, Clone, Abomonation)]
pub(crate) struct Nil;
