use serde::{ Serialize, Serializer };

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBody {
  pub filters: Vec<QueryFilter>,
  #[serde(serialize_with = "ser_query_flags")]
  pub flags: Vec<QueryFlags>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFilter {
  pub criteria: Vec<QueryCriteria>,
}

#[derive(Debug)]
pub enum QueryFlags {
  IncludeFiles,
  IncludeAssetUri,
  IncludeLatestVersionOnly,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryCriteria {
  #[serde(serialize_with = "ser_query_filter_type")]
  pub filter_type: QueryFilterType,
  pub value: String,
}

#[derive(Debug)]
pub enum QueryFilterType {
  ExtensionName,
}

fn ser_query_flags<S: Serializer>(flags: &Vec<QueryFlags>, ser: S) -> Result<S::Ok, S::Error> {
  let mut value = 0;

  for flag in flags.iter() {
    value ^= match flag {
      QueryFlags::IncludeFiles => 0x2,
      QueryFlags::IncludeAssetUri => 0x80,
      QueryFlags::IncludeLatestVersionOnly => 0x200,
    }
  }

  return ser.serialize_i32(value);
}

fn ser_query_filter_type<S: Serializer>(filter_type: &QueryFilterType, ser: S) -> Result<S::Ok, S::Error> {
  let value = match filter_type {
    QueryFilterType::ExtensionName => 7,
  };

  return ser.serialize_i32(value);
}