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
pub struct PayeeLocationResponse {
  #[serde(rename = "data")]
  data: ::models::PayeeLocationResponseData
}

impl PayeeLocationResponse {
  pub fn new(data: ::models::PayeeLocationResponseData) -> PayeeLocationResponse {
    PayeeLocationResponse {
      data: data
    }
  }

  pub fn set_data(&mut self, data: ::models::PayeeLocationResponseData) {
    self.data = data;
  }

  pub fn with_data(mut self, data: ::models::PayeeLocationResponseData) -> PayeeLocationResponse {
    self.data = data;
    self
  }

  pub fn data(&self) -> &::models::PayeeLocationResponseData {
    &self.data
  }


}



