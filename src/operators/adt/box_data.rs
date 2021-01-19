use arcon_macros::Arcon;
use prost::{Message, Oneof};

#[derive(Arcon, Message, Clone)]
#[arcon(reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(oneof = "List", tags = "1, 2")]
    pub(crate) list: Option<List>,
}

#[derive(Oneof, Clone)]
pub(crate) enum List {
    #[prost(message, tag = "1")]
    Cons(Box<Cons>),
    #[prost(message, tag = "2")]
    Nil(Box<Nil>),
}

#[derive(Message, Clone)]
pub(crate) struct Cons {
    #[prost(int32, tag = "3")]
    pub(crate) val: i32,
    #[prost(oneof = "List", tags = "1, 2")]
    pub(crate) tail: Option<List>,
}

#[derive(Message, Clone)]
pub(crate) struct Nil {}
