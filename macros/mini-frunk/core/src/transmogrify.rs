#![forbid(unsafe_code)]

use crate::labelled::LabelledGeneric;
use crate::hlist::{ HCons, HNil };
pub enum Here {}
pub struct There<T>(std::marker::PhantomData<T>);

pub trait Plucker<Target, Indices> {
    type Remainder;
    fn pluck(self) -> (Target, Self::Remainder);
}

impl<Tail, Target> Plucker<Target, Here> for HCons<Target, Tail>
{
    type Remainder = Tail;

    fn pluck(self) -> (Target, Self::Remainder) {
        (self.head, self.tail)
    }
}

impl<Head, Tail, Target, TailIndices> Plucker<Target, There<TailIndices>> for HCons<Head, Tail>
where
    Tail: Plucker<Target, TailIndices>
{
    type Remainder = HCons<Head, <Tail as Plucker<Target, TailIndices>>::Remainder>;

    fn pluck(self) -> (Target, Self::Remainder) {
        let (target, remainder)= Tail::pluck(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: remainder,
            }
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Sculptor<Target, Indices> {
    type Remainder;
    fn sculpt(self) -> (Target, Self::Remainder);
}

impl<Something> Sculptor<HNil, HNil> for Something {
    type Remainder = Something;

    fn sculpt(self) -> (HNil, Self::Remainder) {
        (HNil, self)
    }
}

impl<SrcHead, SrcTail, DstHead, DstTail, HeadIndices, TailIndices> 
    Sculptor<HCons<DstHead, DstTail>, HCons<HeadIndices, TailIndices>> for HCons<SrcHead, SrcTail>
where
    HCons<SrcHead, SrcTail> : Plucker<DstHead, HeadIndices>,
    <HCons<SrcHead, SrcTail> as Plucker<DstHead, HeadIndices>>::Remainder : Sculptor<DstTail, TailIndices>,
{
    type Remainder = <<HCons<SrcHead, SrcTail> as Plucker<DstHead, HeadIndices>>::Remainder 
        as Sculptor<DstTail, TailIndices>>::Remainder;
        
    fn sculpt(self) -> (HCons<DstHead, DstTail>, Self::Remainder) {
        let (h, r): (DstHead, <HCons<SrcHead, SrcTail> as Plucker<DstHead, HeadIndices>>::Remainder) = self.pluck();
        let (tail, remainder): (DstTail, Self::Remainder) = r.sculpt();
        (
            HCons {
                head: h,
                tail,
            },
            remainder
        )

    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Transmogrifier<Dst, Indices> {
    fn transmogrify(self) -> Dst;
}

impl<Dst, Indices, T: LabelledGeneric> Transmogrifier<Dst, Indices> for T 
where   Dst: LabelledGeneric,
        <T as LabelledGeneric>::Repr: Sculptor<<Dst as LabelledGeneric>::Repr, Indices> 
{
    fn transmogrify(self) -> Dst {
        transmogrify_from(self)
    }
}


////////////////////////////////////////////////////////////////////////////////

pub fn transmogrify_from<Src, Dst, Indices>(src: Src) -> Dst
    where Src: LabelledGeneric,
          Dst: LabelledGeneric,
          <Src as LabelledGeneric>::Repr: Sculptor<<Dst as LabelledGeneric>::Repr, Indices> 
{
    let src = Src::into(src);
    let (dst, _) = src.sculpt();
    LabelledGeneric::from(dst)
}
