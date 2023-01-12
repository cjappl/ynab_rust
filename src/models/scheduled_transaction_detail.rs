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
pub struct ScheduledTransactionDetail {
  #[serde(rename = "id")]
  id: String,
  /// The first date for which the Scheduled Transaction was scheduled.
  #[serde(rename = "date_first")]
  date_first: String,
  /// The next date for which the Scheduled Transaction is scheduled.
  #[serde(rename = "date_next")]
  date_next: String,
  #[serde(rename = "frequency")]
  frequency: String,
  /// The scheduled transaction amount in milliunits format
  #[serde(rename = "amount")]
  amount: i64,
  #[serde(rename = "memo")]
  memo: Option<String>,
  /// The scheduled transaction flag
  #[serde(rename = "flag_color")]
  flag_color: Option<String>,
  #[serde(rename = "account_id")]
  account_id: String,
  #[serde(rename = "payee_id")]
  payee_id: Option<String>,
  #[serde(rename = "category_id")]
  category_id: Option<String>,
  /// If a transfer, the account_id which the scheduled transaction transfers to
  #[serde(rename = "transfer_account_id")]
  transfer_account_id: Option<String>,
  /// Whether or not the scheduled transaction has been deleted.  Deleted scheduled transactions will only be included in delta requests.
  #[serde(rename = "deleted")]
  deleted: bool,
  #[serde(rename = "account_name")]
  account_name: String,
  #[serde(rename = "payee_name")]
  payee_name: Option<String>,
  #[serde(rename = "category_name")]
  category_name: Option<String>,
  /// If a split scheduled transaction, the subtransactions.
  #[serde(rename = "subtransactions")]
  subtransactions: Vec<::models::ScheduledSubTransaction>
}

impl ScheduledTransactionDetail {
  pub fn new(id: String, date_first: String, date_next: String, frequency: String, amount: i64, account_id: String, deleted: bool, account_name: String, subtransactions: Vec<::models::ScheduledSubTransaction>) -> ScheduledTransactionDetail {
    ScheduledTransactionDetail {
      id: id,
      date_first: date_first,
      date_next: date_next,
      frequency: frequency,
      amount: amount,
      memo: None,
      flag_color: None,
      account_id: account_id,
      payee_id: None,
      category_id: None,
      transfer_account_id: None,
      deleted: deleted,
      account_name: account_name,
      payee_name: None,
      category_name: None,
      subtransactions: subtransactions
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> ScheduledTransactionDetail {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_date_first(&mut self, date_first: String) {
    self.date_first = date_first;
  }

  pub fn with_date_first(mut self, date_first: String) -> ScheduledTransactionDetail {
    self.date_first = date_first;
    self
  }

  pub fn date_first(&self) -> &String {
    &self.date_first
  }


  pub fn set_date_next(&mut self, date_next: String) {
    self.date_next = date_next;
  }

  pub fn with_date_next(mut self, date_next: String) -> ScheduledTransactionDetail {
    self.date_next = date_next;
    self
  }

  pub fn date_next(&self) -> &String {
    &self.date_next
  }


  pub fn set_frequency(&mut self, frequency: String) {
    self.frequency = frequency;
  }

  pub fn with_frequency(mut self, frequency: String) -> ScheduledTransactionDetail {
    self.frequency = frequency;
    self
  }

  pub fn frequency(&self) -> &String {
    &self.frequency
  }


  pub fn set_amount(&mut self, amount: i64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i64) -> ScheduledTransactionDetail {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i64 {
    &self.amount
  }


  pub fn set_memo(&mut self, memo: String) {
    self.memo = Some(memo);
  }

  pub fn with_memo(mut self, memo: String) -> ScheduledTransactionDetail {
    self.memo = Some(memo);
    self
  }

  pub fn memo(&self) -> Option<&String> {
    self.memo.as_ref()
  }

  pub fn reset_memo(&mut self) {
    self.memo = None;
  }

  pub fn set_flag_color(&mut self, flag_color: String) {
    self.flag_color = Some(flag_color);
  }

  pub fn with_flag_color(mut self, flag_color: String) -> ScheduledTransactionDetail {
    self.flag_color = Some(flag_color);
    self
  }

  pub fn flag_color(&self) -> Option<&String> {
    self.flag_color.as_ref()
  }

  pub fn reset_flag_color(&mut self) {
    self.flag_color = None;
  }

  pub fn set_account_id(&mut self, account_id: String) {
    self.account_id = account_id;
  }

  pub fn with_account_id(mut self, account_id: String) -> ScheduledTransactionDetail {
    self.account_id = account_id;
    self
  }

  pub fn account_id(&self) -> &String {
    &self.account_id
  }


  pub fn set_payee_id(&mut self, payee_id: String) {
    self.payee_id = Some(payee_id);
  }

  pub fn with_payee_id(mut self, payee_id: String) -> ScheduledTransactionDetail {
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

  pub fn with_category_id(mut self, category_id: String) -> ScheduledTransactionDetail {
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

  pub fn with_transfer_account_id(mut self, transfer_account_id: String) -> ScheduledTransactionDetail {
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

  pub fn with_deleted(mut self, deleted: bool) -> ScheduledTransactionDetail {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


  pub fn set_account_name(&mut self, account_name: String) {
    self.account_name = account_name;
  }

  pub fn with_account_name(mut self, account_name: String) -> ScheduledTransactionDetail {
    self.account_name = account_name;
    self
  }

  pub fn account_name(&self) -> &String {
    &self.account_name
  }


  pub fn set_payee_name(&mut self, payee_name: String) {
    self.payee_name = Some(payee_name);
  }

  pub fn with_payee_name(mut self, payee_name: String) -> ScheduledTransactionDetail {
    self.payee_name = Some(payee_name);
    self
  }

  pub fn payee_name(&self) -> Option<&String> {
    self.payee_name.as_ref()
  }

  pub fn reset_payee_name(&mut self) {
    self.payee_name = None;
  }

  pub fn set_category_name(&mut self, category_name: String) {
    self.category_name = Some(category_name);
  }

  pub fn with_category_name(mut self, category_name: String) -> ScheduledTransactionDetail {
    self.category_name = Some(category_name);
    self
  }

  pub fn category_name(&self) -> Option<&String> {
    self.category_name.as_ref()
  }

  pub fn reset_category_name(&mut self) {
    self.category_name = None;
  }

  pub fn set_subtransactions(&mut self, subtransactions: Vec<::models::ScheduledSubTransaction>) {
    self.subtransactions = subtransactions;
  }

  pub fn with_subtransactions(mut self, subtransactions: Vec<::models::ScheduledSubTransaction>) -> ScheduledTransactionDetail {
    self.subtransactions = subtransactions;
    self
  }

  pub fn subtransactions(&self) -> &Vec<::models::ScheduledSubTransaction> {
    &self.subtransactions
  }


}



