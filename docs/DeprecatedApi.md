# \DeprecatedApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_transactions**](DeprecatedApi.md#bulk_create_transactions) | **Post** /budgets/{budget_id}/transactions/bulk | Bulk create transactions


# **bulk_create_transactions**
> ::models::BulkResponse bulk_create_transactions(ctx, budget_id, transactions)
Bulk create transactions

Creates multiple transactions.  Although this endpoint is still supported, it is recommended to use 'POST /budgets/{budget_id}/transactions' to create multiple transactions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **transactions** | [**BulkTransactions**](BulkTransactions.md)| The list of transactions to create | 

### Return type

[**::models::BulkResponse**](BulkResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

