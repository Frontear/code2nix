use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBody {
  pub filters: Vec<QueryFilter>,
  pub flags: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFilter {
  pub criteria: Vec<QueryCriteria>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCriteria {
  pub filter_type: u32,
  pub value: String,
}