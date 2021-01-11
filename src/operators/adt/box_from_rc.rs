use super::{box_data, rc_data};

// Convert from references since it's not possible to own an Rc value

impl<'i> From<&'i rc_data::MyData> for box_data::MyData {
    fn from(data: &'i rc_data::MyData) -> Self {
        box_data::MyData {
            list: data.list.as_ref().map(Into::into),
        }
    }
}

impl<'i> From<&'i rc_data::List> for box_data::List {
    fn from(list: &'i rc_data::List) -> Self {
        match list {
            rc_data::List::Cons(x) => box_data::List::Cons(Box::new(x.as_ref().into())),
            rc_data::List::Nil(x) => box_data::List::Nil(x.into()),
        }
    }
}

impl<'i> From<&'i rc_data::Cons> for box_data::Cons {
    fn from(cons: &'i rc_data::Cons) -> Self {
        box_data::Cons {
            val: cons.val.to_vec(),
            tail: cons.tail.as_ref().map(Into::into),
        }
    }
}

impl<'i> From<&'i rc_data::Nil> for box_data::Nil {
    fn from(_: &'i rc_data::Nil) -> Self {
        box_data::Nil
    }
}
