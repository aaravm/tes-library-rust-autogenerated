# TesListTasksResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tasks** | [**Vec<models::TesTask>**](tesTask.md) | List of tasks. These tasks will be based on the original submitted task document, but with other fields, such as the job state and logging info, added/changed as the job progresses. | 
**next_page_token** | Option<**String**> | Token used to return the next page of results. This value can be used in the `page_token` field of the next ListTasks request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


