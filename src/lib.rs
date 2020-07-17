mod spec;

pub use spec::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_strings(expected: &str, actual: String) {
        eprintln!(
            "============\n  Expected\n============\n\n{}\n\n\
            ==========\n  Actual\n==========\n\n{}\n\n",
            expected, actual
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn rust_empty() {
        let spec = ApiSpec {
            module: "".into(),
            types: vec![],
        };

        compare_strings("", spec.to_rust());
    }

    #[test]
    fn elm_empty() {
        let spec = ApiSpec {
            module: "".into(),
            types: vec![],
        };

        compare_strings("", spec.to_elm());
    }

    fn create_struct_empty() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Struct {
                name: "TestStruct".into(),
                fields: vec![],
            }],
        }
    }

    #[test]
    fn rust_struct_empty() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestStruct {
}";

        compare_strings(expected, create_struct_empty().to_rust());
    }

    #[test]
    fn elm_struct_empty() {
        let expected = "\
type alias TestStruct =
    {}";

        compare_strings(expected, create_struct_empty().to_elm());
    }

    fn create_spec_struct_simple() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Struct {
                name: "TestStruct".into(),
                fields: vec![
                    StructField {
                        name: "foo".into(),
                        data: ("u32".into(), "Int".into()),
                    },
                    StructField {
                        name: "bar".into(),
                        data: ("String".into(), "String".into()),
                    },
                ],
            }],
        }
    }

    #[test]
    fn rust_struct_simple() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestStruct {
    foo: u32,
    bar: String,
}";

        compare_strings(expected, create_spec_struct_simple().to_rust());
    }

    #[test]
    fn elm_struct_simple() {
        let expected = "\
type alias TestStruct =
    { foo: Int
    , bar: String
    }";

        compare_strings(expected, create_spec_struct_simple().to_elm());
    }

    fn create_spec_struct_with_vec() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Struct {
                name: "TestStruct".into(),
                fields: vec![StructField {
                    name: "foo".into(),
                    data: ("Vec<u32>".into(), "List Int".into()),
                }],
            }],
        }
    }

    #[test]
    fn rust_struct_with_vec() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestStruct {
    foo: Vec<u32>,
}";

        compare_strings(expected, create_spec_struct_with_vec().to_rust());
    }

    #[test]
    fn elm_struct_with_vec() {
        let expected = "\
type alias TestStruct =
    { foo: (List Int)
    }";

        compare_strings(expected, create_spec_struct_with_vec().to_elm());
    }

    fn create_spec_enum_empty() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Enum {
                name: "TestEnum".into(),
                variants: vec![],
            }],
        }
    }

    #[test]
    fn rust_enum_empty() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TestEnum {
}";

        compare_strings(expected, create_spec_enum_empty().to_rust());
    }

    #[test]
    fn elm_enum_empty() {
        // TODO: error on empty enum?
        let expected = "\
type TestEnum
    = ";

        compare_strings(expected, create_spec_enum_empty().to_elm());
    }

    fn create_spec_enum_simple() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Enum {
                name: "TestEnum".into(),
                variants: vec![
                    EnumVariant {
                        name: "Foo".into(),
                        data: EnumVariantData::None,
                    },
                    EnumVariant {
                        name: "Bar".into(),
                        data: EnumVariantData::None,
                    },
                    EnumVariant {
                        name: "Qux".into(),
                        data: EnumVariantData::None,
                    },
                ],
            }],
        }
    }

    #[test]
    fn rust_enum_simple() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TestEnum {
    Foo,
    Bar,
    Qux,
}";

        compare_strings(expected, create_spec_enum_simple().to_rust());
    }

    #[test]
    fn elm_enum_simple() {
        let expected = "\
type TestEnum
    = Foo
    | Bar
    | Qux";

        compare_strings(expected, create_spec_enum_simple().to_elm());
    }

    fn create_spec_enum_complex() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Enum {
                name: "TestEnum".into(),
                variants: vec![
                    EnumVariant {
                        name: "Foo".into(),
                        data: EnumVariantData::None,
                    },
                    EnumVariant {
                        name: "Bar".into(),
                        data: EnumVariantData::Single(("bool".into(), "Bool".into())),
                    },
                    EnumVariant {
                        name: "Qux".into(),
                        data: EnumVariantData::Struct(vec![
                            StructField {
                                name: "sub1".into(),
                                data: ("u32".into(), "Int".into()),
                            },
                            StructField {
                                name: "sub2".into(),
                                data: ("String".into(), "String".into()),
                            },
                        ]),
                    },
                ],
            }],
        }
    }

    #[test]
    fn rust_enum_complex() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TestEnum {
    Foo,
    Bar(bool),
    Qux {
        sub1: u32,
        sub2: String,
    },
}";

        compare_strings(expected, create_spec_enum_complex().to_rust());
    }

    #[test]
    fn elm_enum_complex() {
        let expected = "\
type TestEnum
    = Foo
    | Bar Bool
    | Qux { sub1: Int, sub2: String }";

        compare_strings(expected, create_spec_enum_complex().to_elm());
    }

    fn create_spec_enum_with_vec() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Enum {
                name: "TestEnum".into(),
                variants: vec![
                    EnumVariant {
                        name: "Bar".into(),
                        data: EnumVariantData::Single(("Vec<u32>".into(), "List Int".into())),
                    },
                    EnumVariant {
                        name: "Qux".into(),
                        data: EnumVariantData::Struct(vec![StructField {
                            name: "sub1".into(),
                            data: ("Vec<bool>".into(), "List Bool".into()),
                        }]),
                    },
                ],
            }],
        }
    }

    #[test]
    fn rust_enum_with_vec() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TestEnum {
    Bar(Vec<u32>),
    Qux {
        sub1: Vec<bool>,
    },
}";

        compare_strings(expected, create_spec_enum_with_vec().to_rust());
    }

    #[test]
    fn elm_enum_with_vec() {
        let expected = "\
type TestEnum
    = Bar (List Int)
    | Qux { sub1: (List Bool) }";

        compare_strings(expected, create_spec_enum_with_vec().to_elm());
    }
}
