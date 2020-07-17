use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EnumVariantData {
    None,
    Single((String, String)),
    // Tuple(Vec<(String, String)>),
    Struct(Vec<StructField>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub data: (String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub data: EnumVariantData,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
        let types_str = self
            .types
            .iter()
            .flat_map(|t| vec![t.to_elm(), t.to_elm_encoder()])
            .collect::<Vec<_>>()
            .join("\n\n");

        format!(
            "\
import Json.Encode
import Json.Decode exposing ((:=))
import Json.Decode.Extra exposing ((|:))

{types}",
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
                    .map(|field| field.to_rust(1))
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
                let sep = format!("\n{}| ", INDENT);

                let variants_fmt = variants
                    .iter()
                    .map(|var| var.to_elm(1))
                    .collect::<Vec<_>>()
                    .join(&sep);

                format!(
                    "\
type {name}
{indent}= {variants}",
                    name = name,
                    variants = variants_fmt,
                    indent = INDENT,
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

fn elm_json_encoder(elm_type: &str) -> String {
    let supported_types = ["String", "Int", "Float", "Bool", "List"];

    elm_type
        .split(' ')
        .map(|t| {
            if supported_types.contains(&t) {
                format!("Json.Encode.{}", t.to_lowercase())
            } else {
                format!("encode{}", t)
            }
        })
        .collect::<Vec<_>>()
        .join(" <| List.map ")
}

impl StructField {
    pub fn to_rust(&self, indent: usize) -> String {
        format!("{}{}: {},\n", INDENT.repeat(indent), self.name, self.data.0)
    }

    pub fn to_elm(&self, _indent: usize) -> String {
        let elm_type = &self.data.1;

        if elm_type.contains(' ') {
            format!("{}: ({})", self.name, elm_type)
        } else {
            format!("{}: {}", self.name, elm_type)
        }
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

    pub fn to_elm(&self, indent: usize) -> String {
        format!("{}{}", self.name, self.data.to_elm(indent))
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
                    .map(|field| field.to_rust(indent + 1))
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

    pub fn to_elm(&self, indent: usize) -> String {
        match self {
            Self::None => "".into(),
            Self::Single((_, elm_type)) => {
                if elm_type.contains(' ') {
                    format!(" ({})", elm_type)
                } else {
                    format!(" {}", elm_type)
                }
            }
            Self::Struct(fields) => {
                let fields_fmt = fields
                    .iter()
                    .map(|field| field.to_elm(indent + 1))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!(" {{ {fields} }}", fields = fields_fmt)
            }
        }
    }
}
