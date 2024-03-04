#![forbid(unsafe_code)]

#[macro_export]
macro_rules! generate_enums {
    ($($c: ident)*) => {
        $(
            #[allow(non_snake_case, non_camel_case_types)]
            #[derive(PartialEq, Debug, Eq)]
            pub enum $c {}
        )*
    };
}


pub mod symbols {
   generate_enums!(
        a b c d e f g h i j k l m n o p q r s t u v w x y z
        A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
        __
   );
}

////////////////////////////////////////////////////////////////////////////////


#[macro_export]
macro_rules! field {
    ($lit: literal, $value: expr) => {
        { Field {name_type_holder: std::marker::PhantomData, value: $value } }
    };

    (($($c: literal),*), $value: expr) => {
        { Field {name_type_holder: std::marker::PhantomData, value: $value } }
    };
}

pub struct Field<N, T> {
    pub name_type_holder: std::marker::PhantomData<N>,
    pub value: T,
}


