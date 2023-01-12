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
pub struct UserResponseData {
  #[serde(rename = "user")]
  user: ::models::User
}

impl UserResponseData {
  pub fn new(user: ::models::User) -> UserResponseData {
    UserResponseData {
      user: user
    }
  }

  pub fn set_user(&mut self, user: ::models::User) {
    self.user = user;
  }

  pub fn with_user(mut self, user: ::models::User) -> UserResponseData {
    self.user = user;
    self
  }

  pub fn user(&self) -> &::models::User {
    &self.user
  }


}



