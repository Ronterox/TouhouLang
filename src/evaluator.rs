#[macro_export]
macro_rules! parse_number {
    ($value: expr, $num: ty) => {
        match $value {
            $crate::parser::Value::String(s) => s.parse().expect("Expected number"),
            $crate::parser::Value::Number(n) => n as $num,
            _ => panic!("Expected string or number"),
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! parse_value {
    ($value: expr, String) => {
        match $value {
            $crate::parser::Value::String(s) => s,
            $crate::parser::Value::Number(n) => n.to_string(),
            _ => std::panic!("Expected string or number"),
        }
    };
    ($value: expr, i32) => {
        parse_number!($value, i32)
    };
    ($value: expr, f32) => {
        parse_number!($value, f32)
    };
}

#[macro_export]
macro_rules! impl_evaluate {
    (Globals, $($field_name:ident: $field_type:tt,)*) => {
        impl Globals {
            pub fn evaluate(&mut self, objs: $crate::parser::Object) {
                for (key, value) in objs.into_iter() {
                    match key.as_str() {
                        $(stringify!($field_name) => {
                            self.$field_name = $crate::parse_value!(value, $field_type);
                        })*
                        _ => {},
                    }
                }
            }
        }
    };
    ($name: ident, $($field_name:ident: $field_type:tt,)*) => {
        impl $name {
            pub fn evaluate(&mut self, objs: $crate::parser::Object) {
                if let Some(obj) = objs.get(&stringify!($name).to_lowercase()) {
                    match obj {
                       $crate::parser::Value::Object(map) => {
                           for (key, value) in map.into_iter() {
                               match key.as_str() {
                                   $(stringify!($field_name) => {
                                       self.$field_name = $crate::parse_value!(value.clone(), $field_type);
                                   })*
                                   _ => {},
                               }
                           }
                       } ,
                       tt => panic!("Expected {} to be a object but found {tt:?}!", stringify!($name)),
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_struct {
    ($name: ident, $($field_name:ident: $field_type:tt,)*) => {
        $crate::impl_evaluate!($name, $($field_name: $field_type,)*);

        impl $name {
            #[allow(dead_code)]
            fn evaluate_text(&mut self, text: &str) {
                self.evaluate($crate::parser::parse($crate::tokenizer::tokenize(text)));
            }

            #[allow(dead_code)]
            fn from_str(code: &str) -> Self {
                let mut me = Self::default();
                me.evaluate_text(code);
                me
            }
        }
    }
}

#[macro_export]
macro_rules! evaluate {
    (
    $(#[$doc:meta])*
    struct $name: ident {
        $($field_name:ident: $field_type:tt,)*
    }
    ) => {
        $(#[$doc])*
        #[derive(Default)]
        struct $name {
            $($field_name: $field_type,)*
        }

        $crate::impl_struct!($name, $($field_name: $field_type,)*);
    }
}

#[macro_export]
macro_rules! evaluate_derive {
    (
    $(#[$doc:meta])*
    struct $name: ident {
        $($field_name:ident: $field_type:tt,)*
    }
    ) => {
        $crate::impl_struct!($name, $($field_name: $field_type,)*);
    }
}
