use super::{box_data, rc_data};
use std::rc::Rc;

impl From<box_data::MyData> for rc_data::MyData {
    fn from(data: box_data::MyData) -> Self {
        rc_data::MyData {
            list: data.list.map(Into::into),
        }
    }
}

impl From<box_data::List> for rc_data::List {
    fn from(list: box_data::List) -> Self {
        match list {
            box_data::List::Cons(x) => rc_data::List::Cons(Rc::new((*x).into())),
            box_data::List::Nil(x) => rc_data::List::Nil(x.into()),
        }
    }
}

impl From<box_data::Cons> for rc_data::Cons {
    fn from(cons: box_data::Cons) -> Self {
        rc_data::Cons {
            val: ndarray::ArcArray::from(cons.val),
            tail: cons.tail.map(Into::into),
        }
    }
}

impl From<box_data::Nil> for rc_data::Nil {
    fn from(_: box_data::Nil) -> Self {
        rc_data::Nil
    }
}
