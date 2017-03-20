
macro_rules! impl_enum_string {
    (pub enum $name:ident {
        $($variant:ident = $value:expr,)*
    }) => {      
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::de::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
                where D: ::serde::de::Deserializer
            {
                struct Visitor;              
                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter)
                                 -> ::std::fmt::Result
                    {
                        $(
                            let _ = formatter.write_str(concat!($value, "\n"))?;
                        )*
                            Ok(())
                    }

                    fn visit_str<E>(self, value: &str)-> Result<Self::Value, E>
                        where E: ::serde::de::Error
                    {
                        match value {
                            $($value => Ok($name::$variant),)*
                                bad => {
                                let msg = format!("invalid value: {}", bad);
                                    Err(E::custom(msg))
                                },
                        }
                    }
                }
                deserializer.deserialize_str(Visitor)
            }

        }
        
        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer
            {
                match *self {
                    $( $name::$variant => serializer.serialize_str($value), )*
                }
            }
        }
    }
}

macro_rules! impl_enum_u32 {
    (pub enum $name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u32)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                serializer.serialize_u64(*self as u64)
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                struct Visitor;
                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter)
                                 -> ::std::fmt::Result
                    {
                        formatter.write_str("GLenum")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                        where E: ::serde::de::Error
                    {
                        match value {
                            $( $value => Ok($name::$variant), )*
                                bad => {
                                    let msg = format!("invalid value: {}", bad);
                                    Err(E::custom(msg))
                                },
                        }
                    }
                }
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}
