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
pub struct PayeeLocation {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "payee_id")]
  payee_id: String,
  #[serde(rename = "latitude")]
  latitude: String,
  #[serde(rename = "longitude")]
  longitude: String,
  /// Whether or not the payee location has been deleted.  Deleted payee locations will only be included in delta requests.
  #[serde(rename = "deleted")]
  deleted: bool
}

impl PayeeLocation {
  pub fn new(id: String, payee_id: String, latitude: String, longitude: String, deleted: bool) -> PayeeLocation {
    PayeeLocation {
      id: id,
      payee_id: payee_id,
      latitude: latitude,
      longitude: longitude,
      deleted: deleted
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> PayeeLocation {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_payee_id(&mut self, payee_id: String) {
    self.payee_id = payee_id;
  }

  pub fn with_payee_id(mut self, payee_id: String) -> PayeeLocation {
    self.payee_id = payee_id;
    self
  }

  pub fn payee_id(&self) -> &String {
    &self.payee_id
  }


  pub fn set_latitude(&mut self, latitude: String) {
    self.latitude = latitude;
  }

  pub fn with_latitude(mut self, latitude: String) -> PayeeLocation {
    self.latitude = latitude;
    self
  }

  pub fn latitude(&self) -> &String {
    &self.latitude
  }


  pub fn set_longitude(&mut self, longitude: String) {
    self.longitude = longitude;
  }

  pub fn with_longitude(mut self, longitude: String) -> PayeeLocation {
    self.longitude = longitude;
    self
  }

  pub fn longitude(&self) -> &String {
    &self.longitude
  }


  pub fn set_deleted(&mut self, deleted: bool) {
    self.deleted = deleted;
  }

  pub fn with_deleted(mut self, deleted: bool) -> PayeeLocation {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


}


