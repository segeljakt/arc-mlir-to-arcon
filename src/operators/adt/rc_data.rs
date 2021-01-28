use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct MyData {
    pub(crate) list: Rc<List>,
}

#[derive(Clone)]
pub(crate) enum List {
    Cons(Rc<Cons>),
    Nil(Rc<Nil>),
}

#[derive(Clone)]
pub(crate) struct Cons {
    pub(crate) val: i32,
    pub(crate) tail: Rc<List>,
}

#[derive(Clone)]
pub(crate) struct Nil {}
