/* 
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// CurrencyFormat : The currency format setting for the budget.  In some cases the format will not be available and will be specified as null.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyFormat {
  #[serde(rename = "iso_code")]
  iso_code: String,
  #[serde(rename = "example_format")]
  example_format: String,
  #[serde(rename = "decimal_digits")]
  decimal_digits: i32,
  #[serde(rename = "decimal_separator")]
  decimal_separator: String,
  #[serde(rename = "symbol_first")]
  symbol_first: bool,
  #[serde(rename = "group_separator")]
  group_separator: String,
  #[serde(rename = "currency_symbol")]
  currency_symbol: String,
  #[serde(rename = "display_symbol")]
  display_symbol: bool
}

impl CurrencyFormat {
  /// The currency format setting for the budget.  In some cases the format will not be available and will be specified as null.
  pub fn new(iso_code: String, example_format: String, decimal_digits: i32, decimal_separator: String, symbol_first: bool, group_separator: String, currency_symbol: String, display_symbol: bool) -> CurrencyFormat {
    CurrencyFormat {
      iso_code: iso_code,
      example_format: example_format,
      decimal_digits: decimal_digits,
      decimal_separator: decimal_separator,
      symbol_first: symbol_first,
      group_separator: group_separator,
      currency_symbol: currency_symbol,
      display_symbol: display_symbol
    }
  }

  pub fn set_iso_code(&mut self, iso_code: String) {
    self.iso_code = iso_code;
  }

  pub fn with_iso_code(mut self, iso_code: String) -> CurrencyFormat {
    self.iso_code = iso_code;
    self
  }

  pub fn iso_code(&self) -> &String {
    &self.iso_code
  }


  pub fn set_example_format(&mut self, example_format: String) {
    self.example_format = example_format;
  }

  pub fn with_example_format(mut self, example_format: String) -> CurrencyFormat {
    self.example_format = example_format;
    self
  }

  pub fn example_format(&self) -> &String {
    &self.example_format
  }


  pub fn set_decimal_digits(&mut self, decimal_digits: i32) {
    self.decimal_digits = decimal_digits;
  }

  pub fn with_decimal_digits(mut self, decimal_digits: i32) -> CurrencyFormat {
    self.decimal_digits = decimal_digits;
    self
  }

  pub fn decimal_digits(&self) -> &i32 {
    &self.decimal_digits
  }


  pub fn set_decimal_separator(&mut self, decimal_separator: String) {
    self.decimal_separator = decimal_separator;
  }

  pub fn with_decimal_separator(mut self, decimal_separator: String) -> CurrencyFormat {
    self.decimal_separator = decimal_separator;
    self
  }

  pub fn decimal_separator(&self) -> &String {
    &self.decimal_separator
  }


  pub fn set_symbol_first(&mut self, symbol_first: bool) {
    self.symbol_first = symbol_first;
  }

  pub fn with_symbol_first(mut self, symbol_first: bool) -> CurrencyFormat {
    self.symbol_first = symbol_first;
    self
  }

  pub fn symbol_first(&self) -> &bool {
    &self.symbol_first
  }


  pub fn set_group_separator(&mut self, group_separator: String) {
    self.group_separator = group_separator;
  }

  pub fn with_group_separator(mut self, group_separator: String) -> CurrencyFormat {
    self.group_separator = group_separator;
    self
  }

  pub fn group_separator(&self) -> &String {
    &self.group_separator
  }


  pub fn set_currency_symbol(&mut self, currency_symbol: String) {
    self.currency_symbol = currency_symbol;
  }

  pub fn with_currency_symbol(mut self, currency_symbol: String) -> CurrencyFormat {
    self.currency_symbol = currency_symbol;
    self
  }

  pub fn currency_symbol(&self) -> &String {
    &self.currency_symbol
  }


  pub fn set_display_symbol(&mut self, display_symbol: bool) {
    self.display_symbol = display_symbol;
  }

  pub fn with_display_symbol(mut self, display_symbol: bool) -> CurrencyFormat {
    self.display_symbol = display_symbol;
    self
  }

  pub fn display_symbol(&self) -> &bool {
    &self.display_symbol
  }


}


