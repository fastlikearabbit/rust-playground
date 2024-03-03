#![forbid(unsafe_code)]

pub trait Generic {
    type Repr;
    fn into(self) -> Self::Repr;
    fn from(repr: Self::Repr) -> Self;
}

fn generic<Dst, Repr>(repr: Repr) -> Dst 
where
    Dst: Generic<Repr = Repr>,
{ 
    todo!()
}
fn into_generic<Src, Repr>(src: Src) -> Repr
where
    Src: Generic<Repr = Repr>
{ 
    todo!()
}
fn convert_from<Src, Dst, Repr>(src: Src) -> Dst
where
    Src: Generic<Repr = Repr>,
    Dst: Generic<Repr = Repr>,
{ 
    todo!()
}

// TODO: your code goes here.
