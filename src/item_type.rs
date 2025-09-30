use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

macro_rules! define_item_type {
    ($($name:literal => $variant:ident),* $(,)?) => {
        #[derive(EnumString, Display, Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]
        pub enum ItemType {
            $(
                #[strum(serialize = $name)]
                #[serde(rename = $name)]
                $variant,
            )*
        }
    };
}

define_item_type! {
    "mod" => Module,
    "externcrate" => ExternCrate,
    "import" => Import,
    "struct" => Struct,
    "union" => Union,
    "enum" => Enum,
    "fn" => Function,
    "type" => TypeAlias,
    "static" => Static,
    "trait" => Trait,
    "impl" => Impl,
    "tymethod" => TyMethod,
    "method" => Method,
    "structfield" => StructField,
    "variant" => Variant,
    "macro" => Macro,
    "primitive" => Primitive,
    "associatedtype" => AssocType,
    "constant" => Constant,
    "associatedconstant" => AssocConst,
    "foreigntype" => ForeignType,
    "keyword" => Keyword,
    "attr" => ProcAttribute,
    "derive" => ProcDerive,
    "traitalias" => TraitAlias,
    "attribute" => Attribute,
}
