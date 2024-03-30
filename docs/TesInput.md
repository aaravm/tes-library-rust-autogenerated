# TesInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> | REQUIRED, unless \"content\" is set.  URL in long term storage, for example:  - s3://my-object-store/file1  - gs://my-bucket/file2  - file:///path/to/my/file  - /path/to/my/file | [optional]
**path** | **String** | Path of the file inside the container. Must be an absolute path. | 
**r#type** | Option<[**models::TesFileType**](tesFileType.md)> |  | [optional]
**content** | Option<**String**> | File content literal.  Implementations should support a minimum of 128 KiB in this field and may define their own maximum.  UTF-8 encoded  If content is not empty, \"url\" must be ignored. | [optional]
**streamable** | Option<**bool**> | Indicate that a file resource could be accessed using a streaming interface, ie a FUSE mounted s3 object. This flag indicates that using a streaming mount, as opposed to downloading the whole file to the local scratch space, may be faster despite the latency and overhead. This does not mean that the backend will use a streaming interface, as it may not be provided by the vendor, but if the capacity is avalible it can be used without degrading the performance of the underlying program. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


