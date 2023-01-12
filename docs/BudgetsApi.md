# \BudgetsApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_budget_by_id**](BudgetsApi.md#get_budget_by_id) | **Get** /budgets/{budget_id} | Single budget
[**get_budget_settings_by_id**](BudgetsApi.md#get_budget_settings_by_id) | **Get** /budgets/{budget_id}/settings | Budget Settings
[**get_budgets**](BudgetsApi.md#get_budgets) | **Get** /budgets | List budgets


# **get_budget_by_id**
> ::models::BudgetDetailResponse get_budget_by_id(ctx, budget_id, optional)
Single budget

Returns a single budget with all related entities.  This resource is effectively a full budget export.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
 **last_knowledge_of_server** | **i64**| The starting server knowledge.  If provided, only entities that have changed since &#x60;last_knowledge_of_server&#x60; will be included. | 

### Return type

[**::models::BudgetDetailResponse**](BudgetDetailResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_budget_settings_by_id**
> ::models::BudgetSettingsResponse get_budget_settings_by_id(ctx, budget_id)
Budget Settings

Returns settings for a budget

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 

### Return type

[**::models::BudgetSettingsResponse**](BudgetSettingsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_budgets**
> ::models::BudgetSummaryResponse get_budgets(ctx, optional)
List budgets

Returns budgets list with summary information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **include_accounts** | **bool**| Whether to include the list of budget accounts | 

### Return type

[**::models::BudgetSummaryResponse**](BudgetSummaryResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

