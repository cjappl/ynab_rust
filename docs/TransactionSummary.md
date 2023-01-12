# TransactionSummary

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**date** | [***String**](string.md) | The transaction date in ISO format (e.g. 2016-12-01) | [default to null]
**amount** | **i64** | The transaction amount in milliunits format | [default to null]
**memo** | **String** |  | [optional] [default to null]
**cleared** | **String** | The cleared status of the transaction | [default to null]
**approved** | **bool** | Whether or not the transaction is approved | [default to null]
**flag_color** | **String** | The transaction flag | [optional] [default to null]
**account_id** | **String** |  | [default to null]
**payee_id** | **String** |  | [optional] [default to null]
**category_id** | **String** |  | [optional] [default to null]
**transfer_account_id** | **String** | If a transfer transaction, the account to which it transfers | [optional] [default to null]
**transfer_transaction_id** | **String** | If a transfer transaction, the id of transaction on the other side of the transfer | [optional] [default to null]
**matched_transaction_id** | **String** | If transaction is matched, the id of the matched transaction | [optional] [default to null]
**import_id** | **String** | If the transaction was imported, this field is a unique (by account) import identifier.  If this transaction was imported through File Based Import or Direct Import and not through the API, the import_id will have the format: &#39;YNAB:[milliunit_amount]:[iso_date]:[occurrence]&#39;.  For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of &#39;YNAB:-294230:2015-12-30:1&#39;.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be &#39;YNAB:-294230:2015-12-30:2&#39;. | [optional] [default to null]
**import_payee_name** | **String** | If the transaction was imported, the payee name that was used when importing and before applying any payee rename rules | [optional] [default to null]
**import_payee_name_original** | **String** | If the transaction was imported, the original payee name as it appeared on the statement | [optional] [default to null]
**deleted** | **bool** | Whether or not the transaction has been deleted.  Deleted transactions will only be included in delta requests. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


