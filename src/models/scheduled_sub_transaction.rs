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
pub struct ScheduledSubTransaction {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "scheduled_transaction_id")]
  scheduled_transaction_id: String,
  /// The scheduled subtransaction amount in milliunits format
  #[serde(rename = "amount")]
  amount: i64,
  #[serde(rename = "memo")]
  memo: Option<String>,
  #[serde(rename = "payee_id")]
  payee_id: Option<String>,
  #[serde(rename = "category_id")]
  category_id: Option<String>,
  /// If a transfer, the account_id which the scheduled subtransaction transfers to
  #[serde(rename = "transfer_account_id")]
  transfer_account_id: Option<String>,
  /// Whether or not the scheduled subtransaction has been deleted.  Deleted scheduled subtransactions will only be included in delta requests.
  #[serde(rename = "deleted")]
  deleted: bool
}

impl ScheduledSubTransaction {
  pub fn new(id: String, scheduled_transaction_id: String, amount: i64, deleted: bool) -> ScheduledSubTransaction {
    ScheduledSubTransaction {
      id: id,
      scheduled_transaction_id: scheduled_transaction_id,
      amount: amount,
      memo: None,
      payee_id: None,
      category_id: None,
      transfer_account_id: None,
      deleted: deleted
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> ScheduledSubTransaction {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_scheduled_transaction_id(&mut self, scheduled_transaction_id: String) {
    self.scheduled_transaction_id = scheduled_transaction_id;
  }

  pub fn with_scheduled_transaction_id(mut self, scheduled_transaction_id: String) -> ScheduledSubTransaction {
    self.scheduled_transaction_id = scheduled_transaction_id;
    self
  }

  pub fn scheduled_transaction_id(&self) -> &String {
    &self.scheduled_transaction_id
  }


  pub fn set_amount(&mut self, amount: i64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i64) -> ScheduledSubTransaction {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i64 {
    &self.amount
  }


  pub fn set_memo(&mut self, memo: String) {
    self.memo = Some(memo);
  }

  pub fn with_memo(mut self, memo: String) -> ScheduledSubTransaction {
    self.memo = Some(memo);
    self
  }

  pub fn memo(&self) -> Option<&String> {
    self.memo.as_ref()
  }

  pub fn reset_memo(&mut self) {
    self.memo = None;
  }

  pub fn set_payee_id(&mut self, payee_id: String) {
    self.payee_id = Some(payee_id);
  }

  pub fn with_payee_id(mut self, payee_id: String) -> ScheduledSubTransaction {
    self.payee_id = Some(payee_id);
    self
  }

  pub fn payee_id(&self) -> Option<&String> {
    self.payee_id.as_ref()
  }

  pub fn reset_payee_id(&mut self) {
    self.payee_id = None;
  }

  pub fn set_category_id(&mut self, category_id: String) {
    self.category_id = Some(category_id);
  }

  pub fn with_category_id(mut self, category_id: String) -> ScheduledSubTransaction {
    self.category_id = Some(category_id);
    self
  }

  pub fn category_id(&self) -> Option<&String> {
    self.category_id.as_ref()
  }

  pub fn reset_category_id(&mut self) {
    self.category_id = None;
  }

  pub fn set_transfer_account_id(&mut self, transfer_account_id: String) {
    self.transfer_account_id = Some(transfer_account_id);
  }

  pub fn with_transfer_account_id(mut self, transfer_account_id: String) -> ScheduledSubTransaction {
    self.transfer_account_id = Some(transfer_account_id);
    self
  }

  pub fn transfer_account_id(&self) -> Option<&String> {
    self.transfer_account_id.as_ref()
  }

  pub fn reset_transfer_account_id(&mut self) {
    self.transfer_account_id = None;
  }

  pub fn set_deleted(&mut self, deleted: bool) {
    self.deleted = deleted;
  }

  pub fn with_deleted(mut self, deleted: bool) -> ScheduledSubTransaction {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


}



