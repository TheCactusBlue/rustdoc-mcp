use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]

pub enum ItemType {
    #[strum(serialize = "mod")]
    #[serde(rename = "mod")]
    Module,
    #[strum(serialize = "externcrate")]
    #[serde(rename = "externcrate")]
    ExternCrate,
    #[strum(serialize = "import")]
    #[serde(rename = "import")]
    Import,
    #[strum(serialize = "struct")]
    #[serde(rename = "struct")]
    Struct,
    #[strum(serialize = "union")]
    #[serde(rename = "union")]
    Union,
    #[strum(serialize = "enum")]
    #[serde(rename = "enum")]
    Enum,
    #[strum(serialize = "fn")]
    #[serde(rename = "fn")]
    Function,
    #[strum(serialize = "type")]
    #[serde(rename = "type")]
    TypeAlias,
    #[strum(serialize = "static")]
    #[serde(rename = "static")]
    Static,
    #[strum(serialize = "trait")]
    #[serde(rename = "trait")]
    Trait,
    #[strum(serialize = "impl")]
    #[serde(rename = "impl")]
    Impl,
    #[strum(serialize = "tymethod")]
    #[serde(rename = "tymethod")]
    TyMethod,
    #[strum(serialize = "method")]
    #[serde(rename = "method")]
    Method,
    #[strum(serialize = "structfield")]
    #[serde(rename = "structfield")]
    StructField,
    #[strum(serialize = "variant")]
    #[serde(rename = "variant")]
    Variant,
    #[strum(serialize = "macro")]
    #[serde(rename = "macro")]
    Macro,
    #[strum(serialize = "primitive")]
    #[serde(rename = "primitive")]
    Primitive,
    #[strum(serialize = "associatedtype")]
    #[serde(rename = "associatedtype")]
    AssocType,
    #[strum(serialize = "constant")]
    #[serde(rename = "constant")]
    Constant,
    #[strum(serialize = "associatedconstant")]
    #[serde(rename = "associatedconstant")]
    AssocConst,
    #[strum(serialize = "foreigntype")]
    #[serde(rename = "foreigntype")]
    ForeignType,
    #[strum(serialize = "keyword")]
    #[serde(rename = "keyword")]
    Keyword,
    #[strum(serialize = "attr")]
    #[serde(rename = "attr")]
    ProcAttribute,
    #[strum(serialize = "derive")]
    #[serde(rename = "derive")]
    ProcDerive,
    #[strum(serialize = "traitalias")]
    #[serde(rename = "traitalias")]
    TraitAlias,
    #[strum(serialize = "attribute")]
    #[serde(rename = "attribute")]
    Attribute,
}
