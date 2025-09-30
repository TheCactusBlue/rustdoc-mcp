pub mod text;

#[tokio::main]
async fn main() {
    let a = text::fetch_online_docs("rmcp", None).await.unwrap();
    println!("{}", a)
}
