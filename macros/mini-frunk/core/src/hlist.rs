#![forbid(unsafe_code)]

#[derive(PartialEq, Eq, Debug)]
pub struct HNil;

#[derive(PartialEq, Eq, Debug)]
pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

impl HNil {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HNil {
    fn default() -> Self {
        Self::new()
    }
}

impl<H, T> HCons<H, T> {
    pub fn new(head: H, tail: T) -> Self {
        Self { head, tail }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! HList {
    () => {
        $crate::hlist::HNil
    };

    ($t:ty) => {
        $crate::hlist::HCons<$t, HNil>
    };

    ($front: ty, $($rest:ty),+) => {
        $crate::hlist::HCons<$front, $crate::HList![ $($rest),+ ]>
    };
}

#[macro_export]
macro_rules! hlist {
    () => {
        $crate::hlist::HNil
    };

    ($e:expr) => {
        $crate::hlist::HCons {
            head: $e,
            tail: $crate::hlist::HNil,
        }
    };

    ($front: expr, $($rest:expr),+) => {
        $crate::hlist::HCons { 
            head: $front,
            tail: $crate::hlist!($($rest),+) 
        }
    };
}

#[macro_export]
macro_rules! hlist_pat {
    () => {
        $crate::hlist::HNil
    };

    ($i:ident) => {
        $crate::hlist::HCons {
            head: $i,
            tail: $crate::hlist::HNil
        }
    };

    ($front: ident, $($rest:ident),+) => {
        $crate::hlist::HCons {
            head: $front, 
            tail: $crate::hlist_pat!($($rest),+)
        }
    };
}
