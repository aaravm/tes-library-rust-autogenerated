/*
 * Task Execution Service
 *
 * ## Executive Summary The Task Execution Service (TES) API is a standardized schema and API for describing and executing batch execution tasks. A task defines a set of input files, a set of containers and commands to run, a set of output files and some other logging and metadata.  TES servers accept task documents and execute them asynchronously on available compute resources. A TES server could be built on top of a traditional HPC queuing system, such as Grid Engine, Slurm or cloud style compute systems such as AWS Batch or Kubernetes. ## Introduction This document describes the TES API and provides details on the specific endpoints, request formats, and responses. It is intended to provide key information for developers of TES-compatible services as well as clients that will call these TES services. Use cases include:    - Deploying existing workflow engines on new infrastructure. Workflow engines   such as CWL-Tes and Cromwell have extentions for using TES. This will allow   a system engineer to deploy them onto a new infrastructure using a job scheduling   system not previously supported by the engine.    - Developing a custom workflow management system. This API provides a common   interface to asynchronous batch processing capabilities. A developer can write   new tools against this interface and expect them to work using a variety of   backend solutions that all support the same specification.   ## Standards The TES API specification is written in OpenAPI and embodies a RESTful service philosophy. It uses JSON in requests and responses and standard HTTP/HTTPS for information transport. HTTPS should be used rather than plain HTTP except for testing or internal-only purposes. ### Authentication and Authorization Is is envisaged that most TES API instances will require users to authenticate to use the endpoints. However, the decision if authentication is required should be taken by TES API implementers.  If authentication is required, we recommend that TES implementations use an OAuth2  bearer token, although they can choose other mechanisms if appropriate.  Checking that a user is authorized to submit TES requests is a responsibility of TES implementations. ### CORS If TES API implementation is to be used by another website or domain it must implement Cross Origin Resource Sharing (CORS). Please refer to https://w3id.org/ga4gh/product-approval-support/cors for more information about GA4GH’s recommendations and how to implement CORS. 
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// TesTask : Task describes an instance of a task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TesTask {
    /// Task identifier assigned by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::TesState>,
    /// User-provided task name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional user-provided description of task for documentation purposes.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input files that will be used by the task. Inputs will be downloaded and mounted into the executor container as defined by the task request document.
    #[serde(rename = "inputs", skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<models::TesInput>>,
    /// Output files. Outputs will be uploaded from the executor container to long-term storage.
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<models::TesOutput>>,
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<models::TesResources>>,
    /// An array of executors to be run. Each of the executors will run one at a time sequentially. Each executor is a different command that will be run, and each can utilize a different docker image. But each of the executors will see the same mapped inputs and volumes that are declared in the parent CreateTask message.  Execution stops on the first error.
    #[serde(rename = "executors")]
    pub executors: Vec<models::TesExecutor>,
    /// Volumes are directories which may be used to share data between Executors. Volumes are initialized as empty directories by the system when the task starts and are mounted at the same path in each Executor.  For example, given a volume defined at `/vol/A`, executor 1 may write a file to `/vol/A/exec1.out.txt`, then executor 2 may read from that file.  (Essentially, this translates to a `docker run -v` flag where the container path is the same for each executor).
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
    /// A key-value map of arbitrary tags. These can be used to store meta-data and annotations about a task. Example: ``` {   \"tags\" : {       \"WORKFLOW_ID\" : \"cwl-01234\",       \"PROJECT_GROUP\" : \"alice-lab\"   } } ```
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    /// Task logging information. Normally, this will contain only one entry, but in the case where a task fails and is retried, an entry will be appended to this list.
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<models::TesTaskLog>>,
    /// Date + time the task was created, in RFC 3339 format. This is set by the system, not the client.
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
}

impl TesTask {
    /// Task describes an instance of a task.
    pub fn new(executors: Vec<models::TesExecutor>) -> TesTask {
        TesTask {
            id: None,
            state: None,
            name: None,
            description: None,
            inputs: None,
            outputs: None,
            resources: None,
            executors,
            volumes: None,
            tags: None,
            logs: None,
            creation_time: None,
        }
    }
}

