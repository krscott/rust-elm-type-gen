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

        compare_strings(
            "\
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

",
            spec.to_elm(),
        );
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
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

type alias TestStruct =
    { \n    }

encodeTestStruct : TestStruct -> Json.Encode.Value
encodeTestStruct record =
    Json.Encode.object
        [ \n        ]";

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
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

type alias TestStruct =
    { foo: Int
    , bar: String
    }

encodeTestStruct : TestStruct -> Json.Encode.Value
encodeTestStruct record =
    Json.Encode.object
        [ (\"foo\", Json.Encode.int <| record.foo)
        , (\"bar\", Json.Encode.string <| record.bar)
        ]";

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
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

type alias TestStruct =
    { foo: (List Int)
    }

encodeTestStruct : TestStruct -> Json.Encode.Value
encodeTestStruct record =
    Json.Encode.object
        [ (\"foo\", Json.Encode.list <| List.map Json.Encode.int <| record.foo)
        ]";

        compare_strings(expected, create_spec_struct_with_vec().to_elm());
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
#[serde(tag = \"var\", content = \"vardata\")]
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
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

type TestEnum
    = Foo
    | Bar
    | Qux

encodeTestEnum : TestEnum -> Json.Encode.Value
encodeTestEnum var =
    case var of
        Foo ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Foo\" )
                ]
        Bar ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Bar\" )
                ]
        Qux ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Qux\" )
                ]";

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
#[serde(tag = \"var\", content = \"vardata\")]
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
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

type TestEnum
    = Foo
    | Bar Bool
    | Qux { sub1: Int, sub2: String }

encodeTestEnum : TestEnum -> Json.Encode.Value
encodeTestEnum var =
    case var of
        Foo ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Foo\" )
                ]
        Bar value ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Bar\" )
                , ( \"vardata\", Json.Encode.bool <| value )
                ]
        Qux record ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Qux\" )
                , ( \"vardata\", Json.Encode.object
                    [ ( \"sub1\", Json.Encode.int <| record.sub1 )
                    , ( \"sub2\", Json.Encode.string <| record.sub2 )
                    ] )
                ]";

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
#[serde(tag = \"var\", content = \"vardata\")]
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
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

type TestEnum
    = Bar (List Int)
    | Qux { sub1: (List Bool) }

encodeTestEnum : TestEnum -> Json.Encode.Value
encodeTestEnum var =
    case var of
        Bar value ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Bar\" )
                , ( \"vardata\", Json.Encode.list <| List.map Json.Encode.int <| value )
                ]
        Qux record ->
            Json.Encode.object
                [ ( \"var\", Json.Encode.string \"Qux\" )
                , ( \"vardata\", Json.Encode.object
                    [ ( \"sub1\", Json.Encode.list <| List.map Json.Encode.bool <| record.sub1 )
                    ] )
                ]";

        compare_strings(expected, create_spec_enum_with_vec().to_elm());
    }
}
