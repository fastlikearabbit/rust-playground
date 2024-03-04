#![forbid(unsafe_code)]


#[macro_export]
macro_rules! generate_enums {
    ($($c: ident),*) => {
        $(pub enum $c {})*
    };
}


pub mod symbols {
   generate_enums!(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);
   pub enum __ {}
}

////////////////////////////////////////////////////////////////////////////////


#[macro_export]
macro_rules! field {
    (($($c: literal),*), $i: expr) => {
        { Field {name_type_holder: std::marker::PhantomData, value: $i } }
    };
}

pub struct Field<N, T> {
    pub name_type_holder: std::marker::PhantomData<N>,
    pub value: T,
}


