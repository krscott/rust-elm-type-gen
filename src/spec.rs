use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnumVariantData {
    None,
    Single((String, String)),
    // Tuple(Vec<(String, String)>),
    Struct(Vec<StructField>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructField {
    pub name: String,
    pub data: (String, String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub data: EnumVariantData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeSpec {
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiSpec {
    pub module: String,
    pub types: Vec<TypeSpec>,
}

const INDENT: &str = "    ";
const TYPE_DERIVE_HEADER: &str = "#[derive(Debug, serde::Serialize, serde::Deserialize)]";
const SERDE_ENUM_HEADER: &str = "#[serde(tag = \"var\", content = \"vardata\")]";

impl ApiSpec {
    pub fn to_rust(&self) -> String {
        self.types
            .iter()
            .map(|t| t.to_rust())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    pub fn to_elm(&self) -> String {
        let exports_str = self
            .types
            .iter()
            .flat_map(|t| {
                let (name, expose) = match t {
                    TypeSpec::Struct { name, .. } => (name, name.clone()),
                    TypeSpec::Enum { name, .. } => (name, format!("{}(..)", name)),
                };
                vec![
                    expose.clone(),
                    format!("decode{}", name),
                    format!("encode{}", name),
                ]
            })
            .collect::<Vec<_>>()
            .join(", ");

        let types_str = self
            .types
            .iter()
            .flat_map(|t| vec![t.to_elm(), t.to_elm_decoder(), t.to_elm_encoder()])
            .collect::<Vec<_>>()
            .join("\n\n");

        format!(
            "\
module {name} exposing ({exports})

import Json.Decode
import Json.Decode.Extra
import Json.Decode.Pipeline
import Json.Encode
import Json.Encode.Extra

{types}",
            name = self.module,
            exports = exports_str,
            types = types_str
        )
    }
}

impl TypeSpec {
    pub fn to_rust(&self) -> String {
        match self {
            Self::Struct { name, fields } => {
                let fields_fmt = fields
                    .iter()
                    .map(|field| field.to_rust(1, true))
                    .collect::<Vec<_>>()
                    .join("");

                format!(
                    "\
{header}
pub struct {name} {{
{fields}}}",
                    header = TYPE_DERIVE_HEADER,
                    name = name,
                    fields = fields_fmt
                )
            }
            Self::Enum { name, variants } => {
                let variants_fmt = variants
                    .iter()
                    .map(|var| var.to_rust(1))
                    .collect::<Vec<_>>()
                    .join("");

                format!(
                    "\
{header}
{enum_header}
pub enum {name} {{
{variants}}}",
                    header = TYPE_DERIVE_HEADER,
                    enum_header = SERDE_ENUM_HEADER,
                    name = name,
                    variants = variants_fmt
                )
            }
        }
    }

    pub fn to_elm(&self) -> String {
        match self {
            Self::Struct { name, fields } => {
                let sep = format!("\n{}, ", INDENT);

                let fields_fmt = fields
                    .iter()
                    .map(|field| field.to_elm(1))
                    .collect::<Vec<_>>()
                    .join(&sep);

                format!(
                    "\
type alias {name} =
{indent}{{ {fields}
{indent}}}",
                    name = name,
                    fields = fields_fmt,
                    indent = INDENT,
                )
            }
            Self::Enum { name, variants } => {
                let subtypes = variants
                    .iter()
                    .filter_map(|var| {
                        if let EnumVariantData::Struct(fields) = &var.data {
                            let subtype = TypeSpec::Struct {
                                name: format!("{}{}", name, var.name),
                                fields: fields.clone(),
                            };
                            Some(format!(
                                "{}\n\n{}\n\n",
                                subtype.to_elm(),
                                subtype.to_elm_decoder()
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("");

                let sep = format!("\n{}| ", INDENT);

                let variants_fmt = variants
                    .iter()
                    .map(|var| var.to_elm(name))
                    .collect::<Vec<_>>()
                    .join(&sep);

                format!(
                    "\
{subtypes}type {name}
{indent}= {variants}",
                    subtypes = subtypes,
                    name = name,
                    variants = variants_fmt,
                    indent = INDENT,
                )
            }
        }
    }

    pub fn to_elm_decoder(&self) -> String {
        match self {
            Self::Struct { name, fields } => {
                let sep = format!("\n{}", INDENT.repeat(2));

                let field_decoders = fields
                    .iter()
                    .map(|field| format!("|> {}", field.to_elm_decoder()))
                    .collect::<Vec<_>>()
                    .join(&sep);

                format!(
                    "\
decode{name} : Json.Decode.Decoder {name}
decode{name} =
    Json.Decode.succeed {name}
        {fields}",
                    name = name,
                    fields = field_decoders
                )
            }
            Self::Enum { name, variants } => {
                let sep = format!("\n{}, ", INDENT.repeat(2));

                let variant_decoders = variants
                    .iter()
                    .map(|var| var.to_elm_decoder(name))
                    .collect::<Vec<_>>()
                    .join(&sep);

                format!(
                    "\
decode{name} : Json.Decode.Decoder {name}
decode{name} =
    Json.Decode.oneOf
        [ {variants}
        ]",
                    name = name,
                    variants = variant_decoders
                )
            }
        }
    }

    pub fn to_elm_encoder(&self) -> String {
        match self {
            Self::Struct { name, fields } => {
                let sep = format!("\n{}, ", INDENT.repeat(2));

                let field_encoders = fields
                    .iter()
                    .map(|field| field.to_elm_encoder())
                    .collect::<Vec<_>>()
                    .join(&sep);

                format!(
                    "\
encode{name} : {name} -> Json.Encode.Value
encode{name} record =
    Json.Encode.object
        [ {fields}
        ]",
                    name = name,
                    fields = field_encoders
                )
            }
            Self::Enum { name, variants } => {
                let variant_cases = variants
                    .iter()
                    .map(|var| var.to_elm_encoder())
                    .collect::<Vec<_>>()
                    .join("");

                format!(
                    "\
encode{name} : {name} -> Json.Encode.Value
encode{name} var =
    case var of{variants}",
                    name = name,
                    variants = variant_cases
                )
            }
        }
    }
}

fn elm_json_decoder(elm_type: &str) -> String {
    let supported_types = ["String", "Int", "Float", "Bool", "List"];

    let decoders = elm_type
        .split(' ')
        .map(|t| {
            if supported_types.contains(&t) {
                format!("Json.Decode.{}", t.to_lowercase())
            } else if t == "Maybe" {
                String::from("Json.Decode.nullable")
            } else {
                format!("decode{}", t)
            }
        })
        .collect::<Vec<_>>();

    if decoders.len() > 1 {
        format!("({})", decoders.join(" "))
    } else {
        decoders.join(" ")
    }
}

fn elm_json_encoder(elm_type: &str) -> String {
    let supported_types = ["String", "Int", "Float", "Bool", "List"];

    let elm_types_split = elm_type
        .split(' ')
        .map(|t| {
            if supported_types.contains(&t) {
                format!("Json.Encode.{}", t.to_lowercase())
            } else if t == "Maybe" {
                String::from("Json.Encode.Extra.maybe")
            } else {
                format!("encode{}", t)
            }
        })
        .collect::<Vec<_>>();

    if elm_type.starts_with("Maybe ") {
        elm_types_split.join(" ")
    } else {
        elm_types_split.join(" <| List.map ")
    }
}

impl StructField {
    pub fn to_rust(&self, indent: usize, add_pub: bool) -> String {
        format!(
            "{}{}{}: {},\n",
            INDENT.repeat(indent),
            if add_pub { "pub " } else { "" },
            self.name,
            self.data.0
        )
    }

    pub fn to_elm(&self, _indent: usize) -> String {
        let elm_type = &self.data.1;

        if elm_type.contains(' ') {
            format!("{} : ({})", self.name, elm_type)
        } else {
            format!("{} : {}", self.name, elm_type)
        }
    }

    pub fn to_elm_decoder(&self) -> String {
        let elm_type = &self.data.1;

        format!(
            "Json.Decode.Pipeline.required \"{name}\" {decoder}",
            name = self.name,
            decoder = elm_json_decoder(elm_type)
        )
    }

    pub fn to_elm_encoder(&self) -> String {
        let elm_type = &self.data.1;

        format!(
            "(\"{name}\", {encoder} <| record.{name})",
            name = self.name,
            encoder = elm_json_encoder(elm_type)
        )
    }
}

impl EnumVariant {
    pub fn to_rust(&self, indent: usize) -> String {
        format!(
            "{}{}{},\n",
            INDENT.repeat(indent),
            self.name,
            self.data.to_rust(indent)
        )
    }

    pub fn to_elm(&self, parent_type_name: &str) -> String {
        match &self.data {
            EnumVariantData::None => format!("{}", self.name),
            EnumVariantData::Single((_, elm_type)) => {
                if elm_type.contains(' ') {
                    format!("{} ({})", self.name, elm_type)
                } else {
                    format!("{} {}", self.name, elm_type)
                }
            }
            EnumVariantData::Struct(_fields) => {
                format!(
                    "{name} {parent}{name}",
                    name = self.name,
                    parent = parent_type_name
                )

                // let fields_fmt = fields
                //     .iter()
                //     .map(|field| field.to_elm(indent + 1))
                //     .collect::<Vec<_>>()
                //     .join(", ");

                // format!(" {{ {fields} }}", fields = fields_fmt)
            }
        }
    }

    pub fn to_elm_decoder(&self, parent_type_name: &str) -> String {
        match &self.data {
            EnumVariantData::None => format!(
                "Json.Decode.Extra.when (Json.Decode.field \"var\" Json.Decode.string) ((==) \"{name}\") <|\n\
                {indent}Json.Decode.succeed {name}",
                name = self.name,
                indent = INDENT.repeat(3)
            ),
            EnumVariantData::Single((_, elm_type)) => format!(
                "Json.Decode.Extra.when (Json.Decode.field \"var\" Json.Decode.string) ((==) \"{name}\") <|\n\
                {indent}Json.Decode.map {name} (Json.Decode.field \"vardata\" <| {decoder})",
                name = self.name,
                decoder = elm_json_decoder(elm_type),
                indent = INDENT.repeat(3)
            ),
            EnumVariantData::Struct(_) => format!(
                "Json.Decode.Extra.when (Json.Decode.field \"var\" Json.Decode.string) ((==) \"{name}\") <|\n\
                {indent}Json.Decode.map {name} (Json.Decode.field \"vardata\" <| decode{parent}{name})",
                name = self.name,
                indent = INDENT.repeat(3),
                parent = parent_type_name,
            )
        }
    }

    pub fn to_elm_encoder(&self) -> String {
        match &self.data {
            EnumVariantData::None => format!(
                "\n\
                {tab}{tab}{name} ->\n\
                {tab}{tab}{tab}Json.Encode.object\n\
                {tab}{tab}{tab}{tab}[ ( \"var\", Json.Encode.string \"{name}\" )\n\
                {tab}{tab}{tab}{tab}]",
                tab = INDENT,
                name = self.name
            ),
            EnumVariantData::Single((_, elm_type)) => format!(
                "\n\
                {tab}{tab}{name} value ->\n\
                {tab}{tab}{tab}Json.Encode.object\n\
                {tab}{tab}{tab}{tab}[ ( \"var\", Json.Encode.string \"{name}\" )\n\
                {tab}{tab}{tab}{tab}, ( \"vardata\", {encoder} <| value )\n\
                {tab}{tab}{tab}{tab}]",
                tab = INDENT,
                name = self.name,
                encoder = elm_json_encoder(elm_type)
            ),
            EnumVariantData::Struct(fields) => format!(
                "\n\
                {tab}{tab}{name} record ->\n\
                {tab}{tab}{tab}Json.Encode.object\n\
                {tab}{tab}{tab}{tab}[ ( \"var\", Json.Encode.string \"{name}\" )\n\
                {tab}{tab}{tab}{tab}, ( \"vardata\", Json.Encode.object\n\
                {tab}{tab}{tab}{tab}{tab}[{encoder}\n\
                {tab}{tab}{tab}{tab}{tab}] )\n\
                {tab}{tab}{tab}{tab}]",
                tab = INDENT,
                name = self.name,
                encoder = fields
                    .iter()
                    .map(|field| format!(
                        " ( \"{name}\", {encoder} <| record.{name} )",
                        name = field.name,
                        encoder = elm_json_encoder(&field.data.1)
                    ))
                    .collect::<Vec<_>>()
                    .join(&format!("\n{tab}{tab}{tab}{tab}{tab},", tab = INDENT))
            ),
        }
    }
}

impl EnumVariantData {
    pub fn to_rust(&self, indent: usize) -> String {
        match self {
            Self::None => "".into(),
            Self::Single((rust_type, _)) => format!("({})", rust_type),
            Self::Struct(fields) => {
                let fields_fmt = fields
                    .iter()
                    .map(|field| field.to_rust(indent + 1, false))
                    .collect::<Vec<_>>()
                    .join("");

                format!(
                    " {{\n{fields}{indent}}}",
                    fields = fields_fmt,
                    indent = INDENT.repeat(indent)
                )
            }
        }
    }
}
