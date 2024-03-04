#![forbid(unsafe_code)]

pub trait Generic {
    type Repr;
    fn into(self) -> Self::Repr;
    fn from(repr: Self::Repr) -> Self;
}

pub fn from_generic<Dst, R>(gen: R) -> Dst
where
    Dst: Generic<Repr = R>
{ 
    Generic::from(gen)
}

pub fn into_generic<Src, R>(src: Src) -> R
where
    Src: Generic<Repr = R>
{ 
    src.into()
}

pub fn convert_from<Src, Dst, R>(src: Src) -> Dst
where
    Src: Generic<Repr = R>,
    Dst: Generic<Repr = R>,
{ 
    Generic::from(src.into())
}

