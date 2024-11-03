#[macro_export]
macro_rules! parse_number {
    ($value: expr, $num: ty) => {
        match $value {
            Value::String(s) => s.parse().expect("Expected number"),
            Value::Number(n) => n as $num,
            _ => panic!("Expected string or number"),
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! parse_value {
    ($value: expr, String) => {
        match $value {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
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
            pub fn evaluate(&mut self, objs: Vec<Object>) {
                for obj in objs {
                    match obj.id.as_str() {
                        $(stringify!($field_name) => {
                            self.$field_name = $crate::parse_value!(obj.value, $field_type);
                        })*
                        _ => {},
                    }
                }
            }
        }
    };
    ($name: ident, $($field_name:ident: $field_type:tt,)*) => {
        impl $name {
            pub fn evaluate(&mut self, objs: Vec<Object>) {
                if let Some(obj) = objs.iter().find(|o| o.id == stringify!($name).to_lowercase()) {
                    if let Value::List(list) = &obj.value {
                        for obj in list {
                            match obj.id.as_str() {
                                $(stringify!($field_name) => {
                                    self.$field_name = $crate::parse_value!(obj.value.clone(), $field_type);
                                })*
                                _ => {},
                            }
                        }
                    } else {
                        panic!("Expected {} to be a list of args", stringify!($name));
                    }
                }
            }
        }
    };
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

        $crate::impl_evaluate!($name, $($field_name: $field_type,)*);

        impl $name {
            #[allow(dead_code)]
            fn evaluate_text(&mut self, text: &str) {
                self.evaluate(parse(tokenize(text)));
            }

            #[allow(dead_code)]
            fn new(code: &str) -> Self {
                let mut me = Self::default();
                me.evaluate_text(code);
                me
            }
        }
    }
}

