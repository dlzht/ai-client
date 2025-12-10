use ai_client_common::errors::Result;
use ai_client_qianwen::client::QianWenClient;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  if let Err(error) = run_main().await {
    println!("{:?}", error);
  }
}

async fn run_main() -> Result<()> {
  let api_key =
    std::env::var("QIAN_WEN_API_KEY").unwrap_or("sk-00000000000000000000000000000000".to_string());
  println!("api_key: {}", api_key);
  let client = QianWenClient::new(api_key)?;
  let req = client.req_chat_completion_with_user_text("荒野大镖客的英文名称是什么");
  let res = client.chat_completion(&req).await?;
  println!("{:?}", res);
  Ok(())
}
