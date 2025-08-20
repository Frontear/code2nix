mod query;
mod response;

pub use query::*;
pub use response::*;

pub const ENDPOINT: &'static str = "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=7.2-preview.1";