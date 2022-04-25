macro_rules! display_enum {
    ($name: ident, {$($variant: ident => $desc: expr),*}) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match *self {
                    $(
                        $name::$variant => fmt.write_str($desc),
                    )*
                }
            }
        }
    };
}
