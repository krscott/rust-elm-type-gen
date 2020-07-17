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

impl ApiSpec {
    pub fn to_rust(&self) -> String {
        self.types
            .iter()
            .map(|t| t.to_rust())
            .collect::<Vec<_>>()
            .join("\n\n")
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
pub enum {name} {{
{variants}}}",
                    header = TYPE_DERIVE_HEADER,
                    name = name,
                    variants = variants_fmt
                )
            }
        }
    }
}

impl StructField {
    pub fn to_rust(&self, indent: usize) -> String {
        format!("{}{}: {},\n", INDENT.repeat(indent), self.name, self.data.0)
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
}
