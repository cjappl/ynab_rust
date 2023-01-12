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
pub struct CategoryGroupWithCategories {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "name")]
  name: String,
  /// Whether or not the category group is hidden
  #[serde(rename = "hidden")]
  hidden: bool,
  /// Whether or not the category group has been deleted.  Deleted category groups will only be included in delta requests.
  #[serde(rename = "deleted")]
  deleted: bool,
  /// Category group categories.  Amounts (budgeted, activity, balance, etc.) are specific to the current budget month (UTC).
  #[serde(rename = "categories")]
  categories: Vec<::models::Category>
}

impl CategoryGroupWithCategories {
  pub fn new(id: String, name: String, hidden: bool, deleted: bool, categories: Vec<::models::Category>) -> CategoryGroupWithCategories {
    CategoryGroupWithCategories {
      id: id,
      name: name,
      hidden: hidden,
      deleted: deleted,
      categories: categories
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> CategoryGroupWithCategories {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> CategoryGroupWithCategories {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_hidden(&mut self, hidden: bool) {
    self.hidden = hidden;
  }

  pub fn with_hidden(mut self, hidden: bool) -> CategoryGroupWithCategories {
    self.hidden = hidden;
    self
  }

  pub fn hidden(&self) -> &bool {
    &self.hidden
  }


  pub fn set_deleted(&mut self, deleted: bool) {
    self.deleted = deleted;
  }

  pub fn with_deleted(mut self, deleted: bool) -> CategoryGroupWithCategories {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


  pub fn set_categories(&mut self, categories: Vec<::models::Category>) {
    self.categories = categories;
  }

  pub fn with_categories(mut self, categories: Vec<::models::Category>) -> CategoryGroupWithCategories {
    self.categories = categories;
    self
  }

  pub fn categories(&self) -> &Vec<::models::Category> {
    &self.categories
  }


}


