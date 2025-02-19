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

/// TesState : Task state as defined by the server.   - `UNKNOWN`: The state of the task is unknown. The cause for this status   message may be dependent on the underlying system. The `UNKNOWN` states   provides a safe default for messages where this field is missing so   that a missing field does not accidentally imply that   the state is QUEUED.  - `QUEUED`: The task is queued and awaiting resources to begin computing.  - `INITIALIZING`: The task has been assigned to a worker and is currently preparing to run. For example, the worker may be turning on, downloading input files, etc.  - `RUNNING`: The task is running. Input files are downloaded and the first Executor has been started.  - `PAUSED`: The task is paused. The reasons for this would be tied to   the specific system running the job. An implementation may have the ability   to pause a task, but this is not required.  - `COMPLETE`: The task has completed running. Executors have exited without error and output files have been successfully uploaded.  - `EXECUTOR_ERROR`: The task encountered an error in one of the Executor processes. Generally, this means that an Executor exited with a non-zero exit code.  - `SYSTEM_ERROR`: The task was stopped due to a system error, but not from an Executor, for example an upload failed due to network issues, the worker's ran out of disk space, etc.  - `CANCELED`: The task was canceled by the user, and downstream resources have been deleted.  - `CANCELING`: The task was canceled by the user, but the downstream resources are still awaiting deletion.  - `PREEMPTED`: The task is stopped (preempted) by the system. The reasons for this would be tied to the specific system running the job. Generally, this means that the system reclaimed the compute capacity for reallocation.
/// Task state as defined by the server.   - `UNKNOWN`: The state of the task is unknown. The cause for this status   message may be dependent on the underlying system. The `UNKNOWN` states   provides a safe default for messages where this field is missing so   that a missing field does not accidentally imply that   the state is QUEUED.  - `QUEUED`: The task is queued and awaiting resources to begin computing.  - `INITIALIZING`: The task has been assigned to a worker and is currently preparing to run. For example, the worker may be turning on, downloading input files, etc.  - `RUNNING`: The task is running. Input files are downloaded and the first Executor has been started.  - `PAUSED`: The task is paused. The reasons for this would be tied to   the specific system running the job. An implementation may have the ability   to pause a task, but this is not required.  - `COMPLETE`: The task has completed running. Executors have exited without error and output files have been successfully uploaded.  - `EXECUTOR_ERROR`: The task encountered an error in one of the Executor processes. Generally, this means that an Executor exited with a non-zero exit code.  - `SYSTEM_ERROR`: The task was stopped due to a system error, but not from an Executor, for example an upload failed due to network issues, the worker's ran out of disk space, etc.  - `CANCELED`: The task was canceled by the user, and downstream resources have been deleted.  - `CANCELING`: The task was canceled by the user, but the downstream resources are still awaiting deletion.  - `PREEMPTED`: The task is stopped (preempted) by the system. The reasons for this would be tied to the specific system running the job. Generally, this means that the system reclaimed the compute capacity for reallocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TesState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "INITIALIZING")]
    Initializing,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "EXECUTOR_ERROR")]
    ExecutorError,
    #[serde(rename = "SYSTEM_ERROR")]
    SystemError,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "PREEMPTED")]
    Preempted,
    #[serde(rename = "CANCELING")]
    Canceling,

}

impl ToString for TesState {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNOWN"),
            Self::Queued => String::from("QUEUED"),
            Self::Initializing => String::from("INITIALIZING"),
            Self::Running => String::from("RUNNING"),
            Self::Paused => String::from("PAUSED"),
            Self::Complete => String::from("COMPLETE"),
            Self::ExecutorError => String::from("EXECUTOR_ERROR"),
            Self::SystemError => String::from("SYSTEM_ERROR"),
            Self::Canceled => String::from("CANCELED"),
            Self::Preempted => String::from("PREEMPTED"),
            Self::Canceling => String::from("CANCELING"),
        }
    }
}

impl Default for TesState {
    fn default() -> TesState {
        Self::Unknown
    }
}

