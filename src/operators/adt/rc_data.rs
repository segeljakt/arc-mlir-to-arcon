use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct MyData {
    pub(crate) list: Option<List>,
}

#[derive(Clone)]
pub(crate) enum List {
    Cons(Rc<Cons>),
    Nil(Nil),
}

#[derive(Clone)]
pub(crate) struct Cons {
    pub(crate) val: ndarray::ArcArray<i32, ndarray::Dim<[usize; 1]>>,
    pub(crate) tail: Option<List>,
}

#[derive(Clone)]
pub(crate) struct Nil;
