/* 
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkResponseData {
  #[serde(rename = "bulk")]
  bulk: ::models::BulkResponseDataBulk
}

impl BulkResponseData {
  pub fn new(bulk: ::models::BulkResponseDataBulk) -> BulkResponseData {
    BulkResponseData {
      bulk: bulk
    }
  }

  pub fn set_bulk(&mut self, bulk: ::models::BulkResponseDataBulk) {
    self.bulk = bulk;
  }

  pub fn with_bulk(mut self, bulk: ::models::BulkResponseDataBulk) -> BulkResponseData {
    self.bulk = bulk;
    self
  }

  pub fn bulk(&self) -> &::models::BulkResponseDataBulk {
    &self.bulk
  }


}


