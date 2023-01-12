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
pub struct SubTransaction {
  #[serde(rename = "id")]
  id: String,
  #[serde(rename = "transaction_id")]
  transaction_id: String,
  /// The subtransaction amount in milliunits format
  #[serde(rename = "amount")]
  amount: i64,
  #[serde(rename = "memo")]
  memo: Option<String>,
  #[serde(rename = "payee_id")]
  payee_id: Option<String>,
  #[serde(rename = "payee_name")]
  payee_name: Option<String>,
  #[serde(rename = "category_id")]
  category_id: Option<String>,
  #[serde(rename = "category_name")]
  category_name: Option<String>,
  /// If a transfer, the account_id which the subtransaction transfers to
  #[serde(rename = "transfer_account_id")]
  transfer_account_id: Option<String>,
  /// If a transfer, the id of transaction on the other side of the transfer
  #[serde(rename = "transfer_transaction_id")]
  transfer_transaction_id: Option<String>,
  /// Whether or not the subtransaction has been deleted.  Deleted subtransactions will only be included in delta requests.
  #[serde(rename = "deleted")]
  deleted: bool
}

impl SubTransaction {
  pub fn new(id: String, transaction_id: String, amount: i64, deleted: bool) -> SubTransaction {
    SubTransaction {
      id: id,
      transaction_id: transaction_id,
      amount: amount,
      memo: None,
      payee_id: None,
      payee_name: None,
      category_id: None,
      category_name: None,
      transfer_account_id: None,
      transfer_transaction_id: None,
      deleted: deleted
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> SubTransaction {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_transaction_id(&mut self, transaction_id: String) {
    self.transaction_id = transaction_id;
  }

  pub fn with_transaction_id(mut self, transaction_id: String) -> SubTransaction {
    self.transaction_id = transaction_id;
    self
  }

  pub fn transaction_id(&self) -> &String {
    &self.transaction_id
  }


  pub fn set_amount(&mut self, amount: i64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i64) -> SubTransaction {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i64 {
    &self.amount
  }


  pub fn set_memo(&mut self, memo: String) {
    self.memo = Some(memo);
  }

  pub fn with_memo(mut self, memo: String) -> SubTransaction {
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

  pub fn with_payee_id(mut self, payee_id: String) -> SubTransaction {
    self.payee_id = Some(payee_id);
    self
  }

  pub fn payee_id(&self) -> Option<&String> {
    self.payee_id.as_ref()
  }

  pub fn reset_payee_id(&mut self) {
    self.payee_id = None;
  }

  pub fn set_payee_name(&mut self, payee_name: String) {
    self.payee_name = Some(payee_name);
  }

  pub fn with_payee_name(mut self, payee_name: String) -> SubTransaction {
    self.payee_name = Some(payee_name);
    self
  }

  pub fn payee_name(&self) -> Option<&String> {
    self.payee_name.as_ref()
  }

  pub fn reset_payee_name(&mut self) {
    self.payee_name = None;
  }

  pub fn set_category_id(&mut self, category_id: String) {
    self.category_id = Some(category_id);
  }

  pub fn with_category_id(mut self, category_id: String) -> SubTransaction {
    self.category_id = Some(category_id);
    self
  }

  pub fn category_id(&self) -> Option<&String> {
    self.category_id.as_ref()
  }

  pub fn reset_category_id(&mut self) {
    self.category_id = None;
  }

  pub fn set_category_name(&mut self, category_name: String) {
    self.category_name = Some(category_name);
  }

  pub fn with_category_name(mut self, category_name: String) -> SubTransaction {
    self.category_name = Some(category_name);
    self
  }

  pub fn category_name(&self) -> Option<&String> {
    self.category_name.as_ref()
  }

  pub fn reset_category_name(&mut self) {
    self.category_name = None;
  }

  pub fn set_transfer_account_id(&mut self, transfer_account_id: String) {
    self.transfer_account_id = Some(transfer_account_id);
  }

  pub fn with_transfer_account_id(mut self, transfer_account_id: String) -> SubTransaction {
    self.transfer_account_id = Some(transfer_account_id);
    self
  }

  pub fn transfer_account_id(&self) -> Option<&String> {
    self.transfer_account_id.as_ref()
  }

  pub fn reset_transfer_account_id(&mut self) {
    self.transfer_account_id = None;
  }

  pub fn set_transfer_transaction_id(&mut self, transfer_transaction_id: String) {
    self.transfer_transaction_id = Some(transfer_transaction_id);
  }

  pub fn with_transfer_transaction_id(mut self, transfer_transaction_id: String) -> SubTransaction {
    self.transfer_transaction_id = Some(transfer_transaction_id);
    self
  }

  pub fn transfer_transaction_id(&self) -> Option<&String> {
    self.transfer_transaction_id.as_ref()
  }

  pub fn reset_transfer_transaction_id(&mut self) {
    self.transfer_transaction_id = None;
  }

  pub fn set_deleted(&mut self, deleted: bool) {
    self.deleted = deleted;
  }

  pub fn with_deleted(mut self, deleted: bool) -> SubTransaction {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


}



