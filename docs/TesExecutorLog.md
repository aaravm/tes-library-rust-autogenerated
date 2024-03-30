# TesExecutorLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_time** | Option<**String**> | Time the executor started, in RFC 3339 format. | [optional]
**end_time** | Option<**String**> | Time the executor ended, in RFC 3339 format. | [optional]
**stdout** | Option<**String**> | Stdout content.  This is meant for convenience. No guarantees are made about the content. Implementations may chose different approaches: only the head, only the tail, a URL reference only, etc.  In order to capture the full stdout client should set Executor.stdout to a container file path, and use Task.outputs to upload that file to permanent storage. | [optional]
**stderr** | Option<**String**> | Stderr content.  This is meant for convenience. No guarantees are made about the content. Implementations may chose different approaches: only the head, only the tail, a URL reference only, etc.  In order to capture the full stderr client should set Executor.stderr to a container file path, and use Task.outputs to upload that file to permanent storage. | [optional]
**exit_code** | **i32** | Exit code. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


