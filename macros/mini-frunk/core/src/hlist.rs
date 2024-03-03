#![forbid(unsafe_code)]
pub struct HNil;

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
        HNil::new()
    };

    ($t:ty) => {
        HCons<$t, HNil>
    };

    ($front: ty, $($rest:ty),+) => {
        HCons<$front, HList!($($rest),+)>
    };
}

#[macro_export]
macro_rules! hlist {
    () => {
        HNil::new()
    };

    ($e:expr) => {
        HCons::new($e, HNil)
    };

    ($front: expr, $($rest:expr),+) => {
        HCons::new($front, hlist!($($rest),+))
    };
}

#[macro_export]
macro_rules! hlist_pat {
    ($($i: ident),*) => {
        ($($i),*)
    };
}
