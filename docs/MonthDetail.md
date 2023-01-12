# MonthDetail

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**month** | [***String**](string.md) |  | [default to null]
**note** | **String** |  | [optional] [default to null]
**income** | **i64** | The total amount of transactions categorized to &#39;Inflow: Ready to Assign&#39; in the month | [default to null]
**budgeted** | **i64** | The total amount budgeted in the month | [default to null]
**activity** | **i64** | The total amount of transactions in the month, excluding those categorized to &#39;Inflow: Ready to Assign&#39; | [default to null]
**to_be_budgeted** | **i64** | The available amount for &#39;Ready to Assign&#39; | [default to null]
**age_of_money** | **i32** | The Age of Money as of the month | [optional] [default to null]
**deleted** | **bool** | Whether or not the month has been deleted.  Deleted months will only be included in delta requests. | [default to null]
**categories** | [**Vec<::models::Category>**](Category.md) | The budget month categories.  Amounts (budgeted, activity, balance, etc.) are specific to the {month} parameter specified. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


