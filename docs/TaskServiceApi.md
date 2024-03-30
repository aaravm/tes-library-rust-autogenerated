# \TaskServiceApi

All URIs are relative to */ga4gh/tes/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_task**](TaskServiceApi.md#cancel_task) | **POST** /tasks/{id}:cancel | CancelTask
[**create_task**](TaskServiceApi.md#create_task) | **POST** /tasks | CreateTask
[**get_service_info**](TaskServiceApi.md#get_service_info) | **GET** /service-info | GetServiceInfo
[**get_task**](TaskServiceApi.md#get_task) | **GET** /tasks/{id} | GetTask
[**list_tasks**](TaskServiceApi.md#list_tasks) | **GET** /tasks | ListTasks



## cancel_task

> serde_json::Value cancel_task(id)
CancelTask

Cancel a task based on providing an exact task ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of task to be canceled. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_task

> models::TesCreateTaskResponse create_task(body)
CreateTask

Create a new task. The user provides a Task document, which the server uses as a basis and adds additional fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TesTask**](TesTask.md) |  | [required] |

### Return type

[**models::TesCreateTaskResponse**](tesCreateTaskResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_info

> models::TesServiceInfo get_service_info()
GetServiceInfo

Provides information about the service, this structure is based on the standardized GA4GH service info structure. In addition, this endpoint will also provide information about customized storage endpoints offered by the TES server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TesServiceInfo**](tesServiceInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::TesTask get_task(id, view)
GetTask

Get a single task, based on providing the exact task ID string.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of task to retrieve. | [required] |
**view** | Option<**String**> | OPTIONAL. Affects the fields included in the returned Task messages.  `MINIMAL`: Task message will include ONLY the fields: - `tesTask.Id` - `tesTask.State`  `BASIC`: Task message will include all fields EXCEPT: - `tesTask.ExecutorLog.stdout` - `tesTask.ExecutorLog.stderr` - `tesInput.content` - `tesTaskLog.system_logs`  `FULL`: Task message includes all fields. |  |[default to MINIMAL]

### Return type

[**models::TesTask**](tesTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> models::TesListTasksResponse list_tasks(name_prefix, state, tag_key, tag_value, page_size, page_token, view)
ListTasks

List tasks tracked by the TES server. This includes queued, active and completed tasks. How long completed tasks are stored by the system may be dependent on the underlying implementation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_prefix** | Option<**String**> | OPTIONAL. Filter the list to include tasks where the name matches this prefix. If unspecified, no task name filtering is done. |  |
**state** | Option<[**TesState**](.md)> | OPTIONAL. Filter tasks by state. If unspecified, no task state filtering is done. |  |
**tag_key** | Option<[**Vec<String>**](String.md)> | OPTIONAL. Provide key tag to filter. The field tag_key is an array of key values, and will be zipped with an optional tag_value array. So the query: ```   ?tag_key=foo1&tag_value=bar1&tag_key=foo2&tag_value=bar2 ``` Should be constructed into the structure { \"foo1\" : \"bar1\", \"foo2\" : \"bar2\"}  ```   ?tag_key=foo1 ``` Should be constructed into the structure {\"foo1\" : \"\"}  If the tag_value is empty, it will be treated as matching any possible value. If a tag value is provided, both the tag's key and value must be exact matches for a task to be returned. Filter                            Tags                          Match? ---------------------------------------------------------------------- {\"foo\": \"bar\"}                    {\"foo\": \"bar\"}                Yes {\"foo\": \"bar\"}                    {\"foo\": \"bat\"}                No {\"foo\": \"\"}                       {\"foo\": \"\"}                   Yes {\"foo\": \"bar\", \"baz\": \"bat\"}      {\"foo\": \"bar\", \"baz\": \"bat\"}  Yes {\"foo\": \"bar\"}                    {\"foo\": \"bar\", \"baz\": \"bat\"}  Yes {\"foo\": \"bar\", \"baz\": \"bat\"}      {\"foo\": \"bar\"}                No {\"foo\": \"\"}                       {\"foo\": \"bar\"}                Yes {\"foo\": \"\"}                       {}                            No |  |
**tag_value** | Option<[**Vec<String>**](String.md)> | OPTIONAL. The companion value field for tag_key |  |
**page_size** | Option<**i32**> | Optional number of tasks to return in one page. Must be less than 2048. Defaults to 256. |  |
**page_token** | Option<**String**> | OPTIONAL. Page token is used to retrieve the next page of results. If unspecified, returns the first page of results. The value can be found in the `next_page_token` field of the last returned result of ListTasks |  |
**view** | Option<**String**> | OPTIONAL. Affects the fields included in the returned Task messages.  `MINIMAL`: Task message will include ONLY the fields: - `tesTask.Id` - `tesTask.State`  `BASIC`: Task message will include all fields EXCEPT: - `tesTask.ExecutorLog.stdout` - `tesTask.ExecutorLog.stderr` - `tesInput.content` - `tesTaskLog.system_logs`  `FULL`: Task message includes all fields. |  |[default to MINIMAL]

### Return type

[**models::TesListTasksResponse**](tesListTasksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

