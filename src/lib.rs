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

    fn create_spec_empty() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Struct {
                name: "TestStruct".into(),
                fields: vec![],
            }],
        }
    }

    #[test]
    fn rust_empty_struct() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestStruct {
}";

        compare_strings(expected, create_spec_empty().to_rust());
    }

    #[test]
    fn elm_empty_struct() {
        let expected = "\
type alias TestStruct =
    {}";

        compare_strings(expected, create_spec_empty().to_elm());
    }

    fn create_spec_simple_struct() -> ApiSpec {
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
    fn rust_simple_struct() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestStruct {
    foo: u32,
    bar: String,
}";

        compare_strings(expected, create_spec_simple_struct().to_rust());
    }

    #[test]
    fn elm_simple_struct() {
        let expected = "\
type alias TestStruct =
    { foo: Int
    , bar: String
    }";

        compare_strings(expected, create_spec_simple_struct().to_elm());
    }

    fn create_spec_empty_enum() -> ApiSpec {
        ApiSpec {
            module: "".into(),
            types: vec![TypeSpec::Enum {
                name: "TestEnum".into(),
                variants: vec![],
            }],
        }
    }

    #[test]
    fn rust_empty_enum() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TestEnum {
}";

        compare_strings(expected, create_spec_empty_enum().to_rust());
    }

    #[test]
    fn elm_empty_enum() {
        // TODO: error on empty enum?
        let expected = "\
type TestEnum
    = ";

        compare_strings(expected, create_spec_empty_enum().to_elm());
    }

    fn create_spec_simple_enum() -> ApiSpec {
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
    fn rust_simple_enum() {
        let expected = "\
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TestEnum {
    Foo,
    Bar,
    Qux,
}";

        compare_strings(expected, create_spec_simple_enum().to_rust());
    }

    #[test]
    fn elm_simple_enum() {
        let expected = "\
type TestEnum
    = Foo
    | Bar
    | Qux";

        compare_strings(expected, create_spec_simple_enum().to_elm());
    }

    fn create_spec_complex_enum() -> ApiSpec {
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
    fn rust_complex_enum() {
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

        compare_strings(expected, create_spec_complex_enum().to_rust());
    }

    #[test]
    fn elm_complex_enum() {
        let expected = "\
type TestEnum
    = Foo
    | Bar Bool
    | Qux { sub1: Int, sub2: String }";

        compare_strings(expected, create_spec_complex_enum().to_elm());
    }
}
