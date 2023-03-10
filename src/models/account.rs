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
pub struct Account {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "type")]
  _type: ::models::AccountType,
  /// Whether this account is on budget or not
  #[serde(rename = "on_budget")]
  on_budget: bool,
  /// Whether this account is closed or not
  #[serde(rename = "closed")]
  closed: bool,
  #[serde(rename = "note")]
  note: Option<String>,
  /// The current balance of the account in milliunits format
  #[serde(rename = "balance")]
  balance: i64,
  /// The current cleared balance of the account in milliunits format
  #[serde(rename = "cleared_balance")]
  cleared_balance: i64,
  /// The current uncleared balance of the account in milliunits format
  #[serde(rename = "uncleared_balance")]
  uncleared_balance: i64,
  /// The payee id which should be used when transferring to this account
  #[serde(rename = "transfer_payee_id")]
  transfer_payee_id: String,
  /// Whether or not the account is linked to a financial institution for automatic transaction import.
  #[serde(rename = "direct_import_linked")]
  direct_import_linked: Option<bool>,
  /// If an account linked to a financial institution (direct_import_linked=true) and the linked connection is not in a healthy state, this will be true.
  #[serde(rename = "direct_import_in_error")]
  direct_import_in_error: Option<bool>,
  /// Whether or not the account has been deleted.  Deleted accounts will only be included in delta requests.
  #[serde(rename = "deleted")]
  deleted: bool
}

impl Account {
  pub fn new(id: String, name: String, _type: ::models::AccountType, on_budget: bool, closed: bool, balance: i64, cleared_balance: i64, uncleared_balance: i64, transfer_payee_id: String, deleted: bool) -> Account {
    Account {
      id: id,
      name: name,
      _type: _type,
      on_budget: on_budget,
      closed: closed,
      note: None,
      balance: balance,
      cleared_balance: cleared_balance,
      uncleared_balance: uncleared_balance,
      transfer_payee_id: transfer_payee_id,
      direct_import_linked: None,
      direct_import_in_error: None,
      deleted: deleted
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> Account {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Account {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_type(&mut self, _type: ::models::AccountType) {
    self._type = _type;
  }

  pub fn with_type(mut self, _type: ::models::AccountType) -> Account {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &::models::AccountType {
    &self._type
  }


  pub fn set_on_budget(&mut self, on_budget: bool) {
    self.on_budget = on_budget;
  }

  pub fn with_on_budget(mut self, on_budget: bool) -> Account {
    self.on_budget = on_budget;
    self
  }

  pub fn on_budget(&self) -> &bool {
    &self.on_budget
  }


  pub fn set_closed(&mut self, closed: bool) {
    self.closed = closed;
  }

  pub fn with_closed(mut self, closed: bool) -> Account {
    self.closed = closed;
    self
  }

  pub fn closed(&self) -> &bool {
    &self.closed
  }


  pub fn set_note(&mut self, note: String) {
    self.note = Some(note);
  }

  pub fn with_note(mut self, note: String) -> Account {
    self.note = Some(note);
    self
  }

  pub fn note(&self) -> Option<&String> {
    self.note.as_ref()
  }

  pub fn reset_note(&mut self) {
    self.note = None;
  }

  pub fn set_balance(&mut self, balance: i64) {
    self.balance = balance;
  }

  pub fn with_balance(mut self, balance: i64) -> Account {
    self.balance = balance;
    self
  }

  pub fn balance(&self) -> &i64 {
    &self.balance
  }


  pub fn set_cleared_balance(&mut self, cleared_balance: i64) {
    self.cleared_balance = cleared_balance;
  }

  pub fn with_cleared_balance(mut self, cleared_balance: i64) -> Account {
    self.cleared_balance = cleared_balance;
    self
  }

  pub fn cleared_balance(&self) -> &i64 {
    &self.cleared_balance
  }


  pub fn set_uncleared_balance(&mut self, uncleared_balance: i64) {
    self.uncleared_balance = uncleared_balance;
  }

  pub fn with_uncleared_balance(mut self, uncleared_balance: i64) -> Account {
    self.uncleared_balance = uncleared_balance;
    self
  }

  pub fn uncleared_balance(&self) -> &i64 {
    &self.uncleared_balance
  }


  pub fn set_transfer_payee_id(&mut self, transfer_payee_id: String) {
    self.transfer_payee_id = transfer_payee_id;
  }

  pub fn with_transfer_payee_id(mut self, transfer_payee_id: String) -> Account {
    self.transfer_payee_id = transfer_payee_id;
    self
  }

  pub fn transfer_payee_id(&self) -> &String {
    &self.transfer_payee_id
  }


  pub fn set_direct_import_linked(&mut self, direct_import_linked: bool) {
    self.direct_import_linked = Some(direct_import_linked);
  }

  pub fn with_direct_import_linked(mut self, direct_import_linked: bool) -> Account {
    self.direct_import_linked = Some(direct_import_linked);
    self
  }

  pub fn direct_import_linked(&self) -> Option<&bool> {
    self.direct_import_linked.as_ref()
  }

  pub fn reset_direct_import_linked(&mut self) {
    self.direct_import_linked = None;
  }

  pub fn set_direct_import_in_error(&mut self, direct_import_in_error: bool) {
    self.direct_import_in_error = Some(direct_import_in_error);
  }

  pub fn with_direct_import_in_error(mut self, direct_import_in_error: bool) -> Account {
    self.direct_import_in_error = Some(direct_import_in_error);
    self
  }

  pub fn direct_import_in_error(&self) -> Option<&bool> {
    self.direct_import_in_error.as_ref()
  }

  pub fn reset_direct_import_in_error(&mut self) {
    self.direct_import_in_error = None;
  }

  pub fn set_deleted(&mut self, deleted: bool) {
    self.deleted = deleted;
  }

  pub fn with_deleted(mut self, deleted: bool) -> Account {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


}



