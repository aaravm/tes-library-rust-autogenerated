# TesExecutor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | **String** | Name of the container image. The string will be passed as the image argument to the containerization run command. Examples:    - `ubuntu`    - `quay.io/aptible/ubuntu`    - `gcr.io/my-org/my-image`    - `myregistryhost:5000/fedora/httpd:version1.0` | 
**command** | **Vec<String>** | A sequence of program arguments to execute, where the first argument is the program to execute (i.e. argv). Example: ``` {   \"command\" : [\"/bin/md5\", \"/data/file1\"] } ``` | 
**workdir** | Option<**String**> | The working directory that the command will be executed in. If not defined, the system will default to the directory set by the container image. | [optional]
**stdin** | Option<**String**> | Path inside the container to a file which will be piped to the executor's stdin. This must be an absolute path. This mechanism could be used in conjunction with the input declaration to process a data file using a tool that expects STDIN.  For example, to get the MD5 sum of a file by reading it into the STDIN ``` {   \"command\" : [\"/bin/md5\"],   \"stdin\" : \"/data/file1\" } ``` | [optional]
**stdout** | Option<**String**> | Path inside the container to a file where the executor's stdout will be written to. Must be an absolute path. Example: ``` {   \"stdout\" : \"/tmp/stdout.log\" } ``` | [optional]
**stderr** | Option<**String**> | Path inside the container to a file where the executor's stderr will be written to. Must be an absolute path. Example: ``` {   \"stderr\" : \"/tmp/stderr.log\" } ``` | [optional]
**env** | Option<**std::collections::HashMap<String, String>**> | Enviromental variables to set within the container. Example: ``` {   \"env\" : {     \"ENV_CONFIG_PATH\" : \"/data/config.file\",     \"BLASTDB\" : \"/data/GRC38\",     \"HMMERDB\" : \"/data/hmmer\"   } } ``` | [optional]
**ignore_error** | Option<**bool**> | Default behavior of running an array of executors is that execution stops on the first error. If `ignore_error` is `True`, then the runner will record error exit codes, but will continue on to the next tesExecutor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


