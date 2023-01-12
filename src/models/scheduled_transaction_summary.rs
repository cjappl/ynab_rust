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
pub struct ScheduledTransactionSummary {
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
  deleted: bool
}

impl ScheduledTransactionSummary {
  pub fn new(id: String, date_first: String, date_next: String, frequency: String, amount: i64, account_id: String, deleted: bool) -> ScheduledTransactionSummary {
    ScheduledTransactionSummary {
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
      deleted: deleted
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> ScheduledTransactionSummary {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_date_first(&mut self, date_first: String) {
    self.date_first = date_first;
  }

  pub fn with_date_first(mut self, date_first: String) -> ScheduledTransactionSummary {
    self.date_first = date_first;
    self
  }

  pub fn date_first(&self) -> &String {
    &self.date_first
  }


  pub fn set_date_next(&mut self, date_next: String) {
    self.date_next = date_next;
  }

  pub fn with_date_next(mut self, date_next: String) -> ScheduledTransactionSummary {
    self.date_next = date_next;
    self
  }

  pub fn date_next(&self) -> &String {
    &self.date_next
  }


  pub fn set_frequency(&mut self, frequency: String) {
    self.frequency = frequency;
  }

  pub fn with_frequency(mut self, frequency: String) -> ScheduledTransactionSummary {
    self.frequency = frequency;
    self
  }

  pub fn frequency(&self) -> &String {
    &self.frequency
  }


  pub fn set_amount(&mut self, amount: i64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i64) -> ScheduledTransactionSummary {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i64 {
    &self.amount
  }


  pub fn set_memo(&mut self, memo: String) {
    self.memo = Some(memo);
  }

  pub fn with_memo(mut self, memo: String) -> ScheduledTransactionSummary {
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

  pub fn with_flag_color(mut self, flag_color: String) -> ScheduledTransactionSummary {
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

  pub fn with_account_id(mut self, account_id: String) -> ScheduledTransactionSummary {
    self.account_id = account_id;
    self
  }

  pub fn account_id(&self) -> &String {
    &self.account_id
  }


  pub fn set_payee_id(&mut self, payee_id: String) {
    self.payee_id = Some(payee_id);
  }

  pub fn with_payee_id(mut self, payee_id: String) -> ScheduledTransactionSummary {
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

  pub fn with_category_id(mut self, category_id: String) -> ScheduledTransactionSummary {
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

  pub fn with_transfer_account_id(mut self, transfer_account_id: String) -> ScheduledTransactionSummary {
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

  pub fn with_deleted(mut self, deleted: bool) -> ScheduledTransactionSummary {
    self.deleted = deleted;
    self
  }

  pub fn deleted(&self) -> &bool {
    &self.deleted
  }


}



