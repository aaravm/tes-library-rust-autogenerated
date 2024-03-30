# TesResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_cores** | Option<**i32**> | Requested number of CPUs | [optional]
**preemptible** | Option<**bool**> | Define if the task is allowed to run on preemptible compute instances, for example, AWS Spot. This option may have no effect when utilized on some backends that don't have the concept of preemptible jobs. | [optional]
**ram_gb** | Option<**f64**> | Requested RAM required in gigabytes (GB) | [optional]
**disk_gb** | Option<**f64**> | Requested disk size in gigabytes (GB) | [optional]
**zones** | Option<**Vec<String>**> | Request that the task be run in these compute zones. How this string is utilized will be dependent on the backend system. For example, a system based on a cluster queueing system may use this string to define priorty queue to which the job is assigned. | [optional]
**backend_parameters** | Option<**std::collections::HashMap<String, String>**> | Key/value pairs for backend configuration. ServiceInfo shall return a list of keys that a backend supports. Keys are case insensitive. It is expected that clients pass all runtime or hardware requirement key/values that are not mapped to existing tesResources properties to backend_parameters. Backends shall log system warnings if a key is passed that is unsupported. Backends shall not store or return unsupported keys if included in a task. If backend_parameters_strict equals true, backends should fail the task if any key/values are unsupported, otherwise, backends should attempt to run the task Intended uses include VM size selection, coprocessor configuration, etc. Example: ``` {   \"backend_parameters\" : {     \"VmSize\" : \"Standard_D64_v3\"   } } ``` | [optional]
**backend_parameters_strict** | Option<**bool**> | If set to true, backends should fail the task if any backend_parameters key/values are unsupported, otherwise, backends should attempt to run the task | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


