# ScheduledTransactionDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**date_first** | [***String**](string.md) | The first date for which the Scheduled Transaction was scheduled. | [default to null]
**date_next** | [***String**](string.md) | The next date for which the Scheduled Transaction is scheduled. | [default to null]
**frequency** | **String** |  | [default to null]
**amount** | **i64** | The scheduled transaction amount in milliunits format | [default to null]
**memo** | **String** |  | [optional] [default to null]
**flag_color** | **String** | The scheduled transaction flag | [optional] [default to null]
**account_id** | **String** |  | [default to null]
**payee_id** | **String** |  | [optional] [default to null]
**category_id** | **String** |  | [optional] [default to null]
**transfer_account_id** | **String** | If a transfer, the account_id which the scheduled transaction transfers to | [optional] [default to null]
**deleted** | **bool** | Whether or not the scheduled transaction has been deleted.  Deleted scheduled transactions will only be included in delta requests. | [default to null]
**account_name** | **String** |  | [default to null]
**payee_name** | **String** |  | [optional] [default to null]
**category_name** | **String** |  | [optional] [default to null]
**subtransactions** | [**Vec<::models::ScheduledSubTransaction>**](ScheduledSubTransaction.md) | If a split scheduled transaction, the subtransactions. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


