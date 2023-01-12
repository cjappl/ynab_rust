# \TransactionsApi

All URIs are relative to *https://api.youneedabudget.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transaction**](TransactionsApi.md#create_transaction) | **Post** /budgets/{budget_id}/transactions | Create a single transaction or multiple transactions
[**get_transaction_by_id**](TransactionsApi.md#get_transaction_by_id) | **Get** /budgets/{budget_id}/transactions/{transaction_id} | Single transaction
[**get_transactions**](TransactionsApi.md#get_transactions) | **Get** /budgets/{budget_id}/transactions | List transactions
[**get_transactions_by_account**](TransactionsApi.md#get_transactions_by_account) | **Get** /budgets/{budget_id}/accounts/{account_id}/transactions | List account transactions
[**get_transactions_by_category**](TransactionsApi.md#get_transactions_by_category) | **Get** /budgets/{budget_id}/categories/{category_id}/transactions | List category transactions
[**get_transactions_by_payee**](TransactionsApi.md#get_transactions_by_payee) | **Get** /budgets/{budget_id}/payees/{payee_id}/transactions | List payee transactions
[**import_transactions**](TransactionsApi.md#import_transactions) | **Post** /budgets/{budget_id}/transactions/import | Import transactions
[**update_transaction**](TransactionsApi.md#update_transaction) | **Put** /budgets/{budget_id}/transactions/{transaction_id} | Updates an existing transaction
[**update_transactions**](TransactionsApi.md#update_transactions) | **Patch** /budgets/{budget_id}/transactions | Update multiple transactions


# **create_transaction**
> ::models::SaveTransactionsResponse create_transaction(ctx, budget_id, data)
Create a single transaction or multiple transactions

Creates a single transaction or multiple transactions.  If you provide a body containing a `transaction` object, a single transaction will be created and if you provide a body containing a `transactions` array, multiple transactions will be created.  Scheduled transactions cannot be created on this endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **data** | [**SaveTransactionsWrapper**](SaveTransactionsWrapper.md)| The transaction or transactions to create.  To create a single transaction you can specify a value for the &#x60;transaction&#x60; object and to create multiple transactions you can specify an array of &#x60;transactions&#x60;.  It is expected that you will only provide a value for one of these objects. | 

### Return type

[**::models::SaveTransactionsResponse**](SaveTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_by_id**
> ::models::TransactionResponse get_transaction_by_id(ctx, budget_id, transaction_id)
Single transaction

Returns a single transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **transaction_id** | **String**| The id of the transaction | 

### Return type

[**::models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transactions**
> ::models::TransactionsResponse get_transactions(ctx, budget_id, optional)
List transactions

Returns budget transactions

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
 **since_date** | **String**| If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). | 
 **_type** | **String**| If specified, only transactions of the specified type will be included. \&quot;uncategorized\&quot; and \&quot;unapproved\&quot; are currently supported. | 
 **last_knowledge_of_server** | **i64**| The starting server knowledge.  If provided, only entities that have changed since &#x60;last_knowledge_of_server&#x60; will be included. | 

### Return type

[**::models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transactions_by_account**
> ::models::TransactionsResponse get_transactions_by_account(ctx, budget_id, account_id, optional)
List account transactions

Returns all transactions for a specified account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **account_id** | **String**| The id of the account | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
 **account_id** | **String**| The id of the account | 
 **since_date** | **String**| If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). | 
 **_type** | **String**| If specified, only transactions of the specified type will be included. \&quot;uncategorized\&quot; and \&quot;unapproved\&quot; are currently supported. | 
 **last_knowledge_of_server** | **i64**| The starting server knowledge.  If provided, only entities that have changed since &#x60;last_knowledge_of_server&#x60; will be included. | 

### Return type

[**::models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transactions_by_category**
> ::models::HybridTransactionsResponse get_transactions_by_category(ctx, budget_id, category_id, optional)
List category transactions

Returns all transactions for a specified category

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **category_id** | **String**| The id of the category | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
 **category_id** | **String**| The id of the category | 
 **since_date** | **String**| If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). | 
 **_type** | **String**| If specified, only transactions of the specified type will be included. \&quot;uncategorized\&quot; and \&quot;unapproved\&quot; are currently supported. | 
 **last_knowledge_of_server** | **i64**| The starting server knowledge.  If provided, only entities that have changed since &#x60;last_knowledge_of_server&#x60; will be included. | 

### Return type

[**::models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transactions_by_payee**
> ::models::HybridTransactionsResponse get_transactions_by_payee(ctx, budget_id, payee_id, optional)
List payee transactions

Returns all transactions for a specified payee

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **payee_id** | **String**| The id of the payee | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
 **payee_id** | **String**| The id of the payee | 
 **since_date** | **String**| If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). | 
 **_type** | **String**| If specified, only transactions of the specified type will be included. \&quot;uncategorized\&quot; and \&quot;unapproved\&quot; are currently supported. | 
 **last_knowledge_of_server** | **i64**| The starting server knowledge.  If provided, only entities that have changed since &#x60;last_knowledge_of_server&#x60; will be included. | 

### Return type

[**::models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **import_transactions**
> ::models::TransactionsImportResponse import_transactions(ctx, budget_id)
Import transactions

Imports available transactions on all linked accounts for the given budget.  Linked accounts allow transactions to be imported directly from a specified financial institution and this endpoint initiates that import.  Sending a request to this endpoint is the equivalent of clicking \"Import\" on each account in the web application or tapping the \"New Transactions\" banner in the mobile applications.  The response for this endpoint contains the transaction ids that have been imported.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 

### Return type

[**::models::TransactionsImportResponse**](TransactionsImportResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_transaction**
> ::models::TransactionResponse update_transaction(ctx, budget_id, transaction_id, data)
Updates an existing transaction

Updates a single transaction

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **transaction_id** | **String**| The id of the transaction | 
  **data** | [**SaveTransactionWrapper**](SaveTransactionWrapper.md)| The transaction to update | 

### Return type

[**::models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_transactions**
> ::models::SaveTransactionsResponse update_transactions(ctx, budget_id, data)
Update multiple transactions

Updates multiple transactions, by `id` or `import_id`.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **budget_id** | **String**| The id of the budget. \&quot;last-used\&quot; can be used to specify the last used budget and \&quot;default\&quot; can be used if default budget selection is enabled (see: https://api.youneedabudget.com/#oauth-default-budget). | 
  **data** | [**UpdateTransactionsWrapper**](UpdateTransactionsWrapper.md)| The transactions to update. Each transaction must have either an &#x60;id&#x60; or &#x60;import_id&#x60; specified. If &#x60;id&#x60; is specified as null an &#x60;import_id&#x60; value can be provided which will allow transaction(s) to be updated by their &#x60;import_id&#x60;. If an &#x60;id&#x60; is specified, it will always be used for lookup. | 

### Return type

[**::models::SaveTransactionsResponse**](SaveTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

