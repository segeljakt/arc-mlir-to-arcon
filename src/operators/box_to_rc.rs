macro_rules! box_to_rc {
    {
        type $id:ident (of $($rest:tt),*)
    } => {
        enum $id {
            $($rest)*
        }
    };
    {
        $id:ident(Box<$ty:ty>) , $($rest:tt)*
    } => {
        $id(Rc<$ty>) , box_to_rc!($rest)
    };
    {
        $id:ident($ty:ty) , $($rest:tt)*
    } => {
        $id($ty) , box_to_rc!($rest)
    }
}
