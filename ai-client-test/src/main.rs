use ai_client_qianwen::chat::client::QianWenChatClientBuilder;
use ai_client_common::errors::Result;
use ai_client_qianwen::chat::model::QianWenChatModel;
use ai_client_qianwen::chat::request::{MessageParam, QianWenChatReq};
use ai_client_qianwen::image::client::QianWenImageClientBuilder;
use ai_client_qianwen::image::request::{ParameterParam, QianWenImageReq};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  if let Err(error ) = run_main().await {
    println!("{}", error);
  }
}

async fn run_main() -> Result<()>{
  let api_key = std::env::var("QIAN_WEN_API_KEY").unwrap_or("sk-00000000000000000000000000000000".to_string());
  let client = QianWenImageClientBuilder::new(api_key)
      .build()?;
  let req = QianWenImageReq::new("qwen-image-plus", "一副典雅庄重的对联悬挂于厅堂之中，房间是个安静古典的中式布置，桌子上放着一些青花瓷，对联上左书“义本生知人机同道善思新”，右书“通云赋智乾坤启数高志远”， 横批“智启通义”，字体飘逸，在中间挂着一幅中国风的画作，内容是岳阳楼。")
      .with_parameters(ParameterParam::new().with_watermark(true));
  let res = client.generate_image(&req).await;
  println!("{:?}", res);
  Ok(())
}