# TesTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Task identifier assigned by the server. | [optional][readonly]
**state** | Option<[**models::TesState**](tesState.md)> |  | [optional]
**name** | Option<**String**> | User-provided task name. | [optional]
**description** | Option<**String**> | Optional user-provided description of task for documentation purposes. | [optional]
**inputs** | Option<[**Vec<models::TesInput>**](tesInput.md)> | Input files that will be used by the task. Inputs will be downloaded and mounted into the executor container as defined by the task request document. | [optional]
**outputs** | Option<[**Vec<models::TesOutput>**](tesOutput.md)> | Output files. Outputs will be uploaded from the executor container to long-term storage. | [optional]
**resources** | Option<[**models::TesResources**](tesResources.md)> |  | [optional]
**executors** | [**Vec<models::TesExecutor>**](tesExecutor.md) | An array of executors to be run. Each of the executors will run one at a time sequentially. Each executor is a different command that will be run, and each can utilize a different docker image. But each of the executors will see the same mapped inputs and volumes that are declared in the parent CreateTask message.  Execution stops on the first error. | 
**volumes** | Option<**Vec<String>**> | Volumes are directories which may be used to share data between Executors. Volumes are initialized as empty directories by the system when the task starts and are mounted at the same path in each Executor.  For example, given a volume defined at `/vol/A`, executor 1 may write a file to `/vol/A/exec1.out.txt`, then executor 2 may read from that file.  (Essentially, this translates to a `docker run -v` flag where the container path is the same for each executor). | [optional]
**tags** | Option<**std::collections::HashMap<String, String>**> | A key-value map of arbitrary tags. These can be used to store meta-data and annotations about a task. Example: ``` {   \"tags\" : {       \"WORKFLOW_ID\" : \"cwl-01234\",       \"PROJECT_GROUP\" : \"alice-lab\"   } } ``` | [optional]
**logs** | Option<[**Vec<models::TesTaskLog>**](tesTaskLog.md)> | Task logging information. Normally, this will contain only one entry, but in the case where a task fails and is retried, an entry will be appended to this list. | [optional][readonly]
**creation_time** | Option<**String**> | Date + time the task was created, in RFC 3339 format. This is set by the system, not the client. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


