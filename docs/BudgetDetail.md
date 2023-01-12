# BudgetDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**name** | **String** |  | [default to null]
**last_modified_on** | **String** | The last time any changes were made to the budget from either a web or mobile client | [optional] [default to null]
**first_month** | [***String**](string.md) | The earliest budget month | [optional] [default to null]
**last_month** | [***String**](string.md) | The latest budget month | [optional] [default to null]
**date_format** | [***::models::DateFormat**](DateFormat.md) |  | [optional] [default to null]
**currency_format** | [***::models::CurrencyFormat**](CurrencyFormat.md) |  | [optional] [default to null]
**accounts** | [**Vec<::models::Account>**](Account.md) |  | [optional] [default to null]
**payees** | [**Vec<::models::Payee>**](Payee.md) |  | [optional] [default to null]
**payee_locations** | [**Vec<::models::PayeeLocation>**](PayeeLocation.md) |  | [optional] [default to null]
**category_groups** | [**Vec<::models::CategoryGroup>**](CategoryGroup.md) |  | [optional] [default to null]
**categories** | [**Vec<::models::Category>**](Category.md) |  | [optional] [default to null]
**months** | [**Vec<::models::MonthDetail>**](MonthDetail.md) |  | [optional] [default to null]
**transactions** | [**Vec<::models::TransactionSummary>**](TransactionSummary.md) |  | [optional] [default to null]
**subtransactions** | [**Vec<::models::SubTransaction>**](SubTransaction.md) |  | [optional] [default to null]
**scheduled_transactions** | [**Vec<::models::ScheduledTransactionSummary>**](ScheduledTransactionSummary.md) |  | [optional] [default to null]
**scheduled_subtransactions** | [**Vec<::models::ScheduledSubTransaction>**](ScheduledSubTransaction.md) |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


