#![forbid(unsafe_code)]
pub trait LabelledGeneric {
    type Repr;
    fn into(self) -> Self::Repr;
    fn from(repr: Self::Repr) -> Self;
}

pub fn from_labelled_generic<Dst, R>(repr: R) -> Dst
where
    Dst: LabelledGeneric<Repr = R>,
{
    LabelledGeneric::from(repr)
}
pub fn into_labelled_generic<Src, R>(src: Src) -> R
where
    Src: LabelledGeneric<Repr = R>,
{
    src.into()
}

pub fn labelled_convert_from<Src, Dst, R>(src: Src) -> Dst
where
    Src: LabelledGeneric<Repr = R>,
    Dst: LabelledGeneric<Repr = R>,
{
    LabelledGeneric::from(src.into())
}
