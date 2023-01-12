# ScheduledSubTransaction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**scheduled_transaction_id** | **String** |  | [default to null]
**amount** | **i64** | The scheduled subtransaction amount in milliunits format | [default to null]
**memo** | **String** |  | [optional] [default to null]
**payee_id** | **String** |  | [optional] [default to null]
**category_id** | **String** |  | [optional] [default to null]
**transfer_account_id** | **String** | If a transfer, the account_id which the scheduled subtransaction transfers to | [optional] [default to null]
**deleted** | **bool** | Whether or not the scheduled subtransaction has been deleted.  Deleted scheduled subtransactions will only be included in delta requests. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


