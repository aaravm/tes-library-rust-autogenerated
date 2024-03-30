# TesTaskLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**logs** | [**Vec<models::TesExecutorLog>**](tesExecutorLog.md) | Logs for each executor | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | Arbitrary logging metadata included by the implementation. | [optional]
**start_time** | Option<**String**> | When the task started, in RFC 3339 format. | [optional]
**end_time** | Option<**String**> | When the task ended, in RFC 3339 format. | [optional]
**outputs** | [**Vec<models::TesOutputFileLog>**](tesOutputFileLog.md) | Information about all output files. Directory outputs are flattened into separate items. | 
**system_logs** | Option<**Vec<String>**> | System logs are any logs the system decides are relevant, which are not tied directly to an Executor process. Content is implementation specific: format, size, etc.  System logs may be collected here to provide convenient access.  For example, the system may include the name of the host where the task is executing, an error message that caused a SYSTEM_ERROR state (e.g. disk is full), etc.  System logs are only included in the FULL task view. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


