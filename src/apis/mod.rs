use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod accounts_api;
pub use self::accounts_api::{ AccountsApi, AccountsApiClient };
mod budgets_api;
pub use self::budgets_api::{ BudgetsApi, BudgetsApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod deprecated_api;
pub use self::deprecated_api::{ DeprecatedApi, DeprecatedApiClient };
mod months_api;
pub use self::months_api::{ MonthsApi, MonthsApiClient };
mod payee_locations_api;
pub use self::payee_locations_api::{ PayeeLocationsApi, PayeeLocationsApiClient };
mod payees_api;
pub use self::payees_api::{ PayeesApi, PayeesApiClient };
mod scheduled_transactions_api;
pub use self::scheduled_transactions_api::{ ScheduledTransactionsApi, ScheduledTransactionsApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };

pub mod configuration;
pub mod client;
