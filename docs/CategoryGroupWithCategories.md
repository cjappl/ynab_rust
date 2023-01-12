# CategoryGroupWithCategories

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**name** | **String** |  | [default to null]
**hidden** | **bool** | Whether or not the category group is hidden | [default to null]
**deleted** | **bool** | Whether or not the category group has been deleted.  Deleted category groups will only be included in delta requests. | [default to null]
**categories** | [**Vec<::models::Category>**](Category.md) | Category group categories.  Amounts (budgeted, activity, balance, etc.) are specific to the current budget month (UTC). | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


