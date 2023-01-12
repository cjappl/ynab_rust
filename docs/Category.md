# Category

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [default to null]
**category_group_id** | **String** |  | [default to null]
**name** | **String** |  | [default to null]
**hidden** | **bool** | Whether or not the category is hidden | [default to null]
**original_category_group_id** | **String** | If category is hidden this is the id of the category group it originally belonged to before it was hidden. | [optional] [default to null]
**note** | **String** |  | [optional] [default to null]
**budgeted** | **i64** | Budgeted amount in milliunits format | [default to null]
**activity** | **i64** | Activity amount in milliunits format | [default to null]
**balance** | **i64** | Balance in milliunits format | [default to null]
**goal_type** | **String** | The type of goal, if the category has a goal (TB&#x3D;&#39;Target Category Balance&#39;, TBD&#x3D;&#39;Target Category Balance by Date&#39;, MF&#x3D;&#39;Monthly Funding&#39;, NEED&#x3D;&#39;Plan Your Spending&#39;) | [optional] [default to null]
**goal_creation_month** | [***String**](string.md) | The month a goal was created | [optional] [default to null]
**goal_target** | **i64** | The goal target amount in milliunits | [optional] [default to null]
**goal_target_month** | [***String**](string.md) | The original target month for the goal to be completed.  Only some goal types specify this date. | [optional] [default to null]
**goal_percentage_complete** | **i32** | The percentage completion of the goal | [optional] [default to null]
**goal_months_to_budget** | **i32** | The number of months, including the current month, left in the current goal period. | [optional] [default to null]
**goal_under_funded** | **i64** | The amount of funding still needed in the current month to stay on track towards completing the goal within the current goal period.  This amount will generally correspond to the &#39;Underfunded&#39; amount in the web and mobile clients except when viewing a category with a Needed for Spending Goal in a future month.  The web and mobile clients will ignore any funding from a prior goal period when viewing category with a Needed for Spending Goal in a future month. | [optional] [default to null]
**goal_overall_funded** | **i64** | The total amount funded towards the goal within the current goal period. | [optional] [default to null]
**goal_overall_left** | **i64** | The amount of funding still needed to complete the goal within the current goal period. | [optional] [default to null]
**deleted** | **bool** | Whether or not the category has been deleted.  Deleted categories will only be included in delta requests. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


