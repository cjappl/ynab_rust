# BudgetSummary

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
**accounts** | [**Vec<::models::Account>**](Account.md) | The budget accounts (only included if &#x60;include_accounts&#x3D;true&#x60; specified as query parameter) | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


