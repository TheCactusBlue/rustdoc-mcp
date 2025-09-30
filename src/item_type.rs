use strum_macros::{Display, EnumString};

#[derive(EnumString, Display, Copy, Clone, PartialEq, Eq, Debug)]

pub enum ItemType {
    #[strum(serialize = "mod")]
    Module,
    #[strum(serialize = "externcrate")]
    ExternCrate,
    #[strum(serialize = "import")]
    Import,
    #[strum(serialize = "struct")]
    Struct,
    #[strum(serialize = "union")]
    Union,
    #[strum(serialize = "enum")]
    Enum,
    #[strum(serialize = "fn")]
    Function,
    #[strum(serialize = "type")]
    TypeAlias,
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "trait")]
    Trait,
    #[strum(serialize = "impl")]
    Impl,
    #[strum(serialize = "tymethod")]
    TyMethod,
    #[strum(serialize = "method")]
    Method,
    #[strum(serialize = "structfield")]
    StructField,
    #[strum(serialize = "variant")]
    Variant,
    #[strum(serialize = "macro")]
    Macro,
    #[strum(serialize = "primitive")]
    Primitive,
    #[strum(serialize = "associatedtype")]
    AssocType,
    #[strum(serialize = "constant")]
    Constant,
    #[strum(serialize = "associatedconstant")]
    AssocConst,
    #[strum(serialize = "foreigntype")]
    ForeignType,
    #[strum(serialize = "keyword")]
    Keyword,
    #[strum(serialize = "attr")]
    ProcAttribute,
    #[strum(serialize = "derive")]
    ProcDerive,
    #[strum(serialize = "traitalias")]
    TraitAlias,
    #[strum(serialize = "attribute")]
    Attribute,
}
