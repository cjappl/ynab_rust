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
pub struct ScheduledTransactionResponseData {
  #[serde(rename = "scheduled_transaction")]
  scheduled_transaction: ::models::ScheduledTransactionDetail
}

impl ScheduledTransactionResponseData {
  pub fn new(scheduled_transaction: ::models::ScheduledTransactionDetail) -> ScheduledTransactionResponseData {
    ScheduledTransactionResponseData {
      scheduled_transaction: scheduled_transaction
    }
  }

  pub fn set_scheduled_transaction(&mut self, scheduled_transaction: ::models::ScheduledTransactionDetail) {
    self.scheduled_transaction = scheduled_transaction;
  }

  pub fn with_scheduled_transaction(mut self, scheduled_transaction: ::models::ScheduledTransactionDetail) -> ScheduledTransactionResponseData {
    self.scheduled_transaction = scheduled_transaction;
    self
  }

  pub fn scheduled_transaction(&self) -> &::models::ScheduledTransactionDetail {
    &self.scheduled_transaction
  }


}



