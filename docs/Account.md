# Account

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**name** | **String** |  | [default to null]
**_type** | [***::models::AccountType**](AccountType.md) |  | [default to null]
**on_budget** | **bool** | Whether this account is on budget or not | [default to null]
**closed** | **bool** | Whether this account is closed or not | [default to null]
**note** | **String** |  | [optional] [default to null]
**balance** | **i64** | The current balance of the account in milliunits format | [default to null]
**cleared_balance** | **i64** | The current cleared balance of the account in milliunits format | [default to null]
**uncleared_balance** | **i64** | The current uncleared balance of the account in milliunits format | [default to null]
**transfer_payee_id** | **String** | The payee id which should be used when transferring to this account | [default to null]
**direct_import_linked** | **bool** | Whether or not the account is linked to a financial institution for automatic transaction import. | [optional] [default to null]
**direct_import_in_error** | **bool** | If an account linked to a financial institution (direct_import_linked&#x3D;true) and the linked connection is not in a healthy state, this will be true. | [optional] [default to null]
**deleted** | **bool** | Whether or not the account has been deleted.  Deleted accounts will only be included in delta requests. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


