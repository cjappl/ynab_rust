# SaveTransactionsResponseData

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_ids** | **Vec<String>** | The transaction ids that were saved | [default to null]
**transaction** | [***::models::TransactionDetail**](TransactionDetail.md) | If a single transaction was specified, the transaction that was saved | [optional] [default to null]
**transactions** | [**Vec<::models::TransactionDetail>**](TransactionDetail.md) | If multiple transactions were specified, the transactions that were saved | [optional] [default to null]
**duplicate_import_ids** | **Vec<String>** | If multiple transactions were specified, a list of import_ids that were not created because of an existing &#x60;import_id&#x60; found on the same account | [optional] [default to null]
**server_knowledge** | **i64** | The knowledge of the server | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


