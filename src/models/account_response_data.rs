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
pub struct AccountResponseData {
  #[serde(rename = "account")]
  account: ::models::Account
}

impl AccountResponseData {
  pub fn new(account: ::models::Account) -> AccountResponseData {
    AccountResponseData {
      account: account
    }
  }

  pub fn set_account(&mut self, account: ::models::Account) {
    self.account = account;
  }

  pub fn with_account(mut self, account: ::models::Account) -> AccountResponseData {
    self.account = account;
    self
  }

  pub fn account(&self) -> &::models::Account {
    &self.account
  }


}



