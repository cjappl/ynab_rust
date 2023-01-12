# SubTransaction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**transaction_id** | **String** |  | [default to null]
**amount** | **i64** | The subtransaction amount in milliunits format | [default to null]
**memo** | **String** |  | [optional] [default to null]
**payee_id** | **String** |  | [optional] [default to null]
**payee_name** | **String** |  | [optional] [default to null]
**category_id** | **String** |  | [optional] [default to null]
**category_name** | **String** |  | [optional] [default to null]
**transfer_account_id** | **String** | If a transfer, the account_id which the subtransaction transfers to | [optional] [default to null]
**transfer_transaction_id** | **String** | If a transfer, the id of transaction on the other side of the transfer | [optional] [default to null]
**deleted** | **bool** | Whether or not the subtransaction has been deleted.  Deleted subtransactions will only be included in delta requests. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


