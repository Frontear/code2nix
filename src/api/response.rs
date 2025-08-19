use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
  pub results: Vec<ResponseResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseResult {
  pub extensions: Vec<ResponseExtension>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseExtension {
  pub extension_name: String,
  pub publisher: ResponsePublisher,
  pub versions: Vec<ResponseVersion>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePublisher {
  pub publisher_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseVersion {
  pub asset_uri: String,
  #[serde(default)]
  pub target_platform: String,
  pub version: String,
}