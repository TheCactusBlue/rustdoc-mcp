use rmcp::{
    handler::server::{
        ServerHandler,
        tool::{Parameters, ToolRouter},
    },
    model::*,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::item_type::ItemType;

#[derive(Clone)]
pub struct RustdocServer {
    tool_router: ToolRouter<Self>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FetchDocsRequest {
    /// The name of the crate to fetch documentation for (e.g. my_crate::submodule::MyStruct)
    path: String,
    /// Optional module name within the crate
    item_type: ItemType,
}

#[tool_router]
impl RustdocServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Fetch Rust documentation from docs.rs as Markdown. Provides documentation for Rust crates, modules, structs, enums, traits, functions, and more."
    )]
    async fn fetch_docs(
        &self,
        params: Parameters<FetchDocsRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let req = params.0;

        match crate::text::rustdoc_fetch(&req.path, Some(req.item_type)).await {
            Ok(docs) => Ok(CallToolResult::success(vec![Content::text(docs)])),
            Err(e) => Err(ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to fetch documentation: {}", e),
                None,
            )),
        }
    }
}

#[tool_handler]
impl ServerHandler for RustdocServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides tools for fetching docs from Docs.rs Tools: fetch_docs."
                    .to_string(),
            ),
        }
    }
}
