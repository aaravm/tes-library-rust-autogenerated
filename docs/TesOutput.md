# TesOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | User-provided name of output file | [optional]
**description** | Option<**String**> | Optional users provided description field, can be used for documentation. | [optional]
**url** | **String** | URL at which the TES server makes the output accessible after the task is complete. When tesOutput.path contains wildcards, it must be a directory; see `tesOutput.path_prefix` for details on how output URLs are constructed in this case. For Example:  - `s3://my-object-store/file1`  - `gs://my-bucket/file2`  - `file:///path/to/my/file` | 
**path** | **String** | Absolute path of the file inside the container. May contain pattern matching wildcards to select multiple outputs at once, but mind implications for `tesOutput.url` and `tesOutput.path_prefix`. Only wildcards defined in IEEE Std 1003.1-2017 (POSIX), 12.3 are supported; see https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_13 | 
**path_prefix** | Option<**String**> | Prefix to be removed from matching outputs if `tesOutput.path` contains wildcards; output URLs are constructed by appending pruned paths to the directory specfied in `tesOutput.url`. Required if `tesOutput.path` contains wildcards, ignored otherwise. | [optional]
**r#type** | Option<[**models::TesFileType**](tesFileType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


