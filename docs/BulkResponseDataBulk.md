# BulkResponseDataBulk

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_ids** | **Vec<String>** | The list of Transaction ids that were created. | [default to null]
**duplicate_import_ids** | **Vec<String>** | If any Transactions were not created because they had an &#x60;import_id&#x60; matching a transaction already on the same account, the specified import_id(s) will be included in this list. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


