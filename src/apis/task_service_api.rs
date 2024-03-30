/*
 * Task Execution Service
 *
 * ## Executive Summary The Task Execution Service (TES) API is a standardized schema and API for describing and executing batch execution tasks. A task defines a set of input files, a set of containers and commands to run, a set of output files and some other logging and metadata.  TES servers accept task documents and execute them asynchronously on available compute resources. A TES server could be built on top of a traditional HPC queuing system, such as Grid Engine, Slurm or cloud style compute systems such as AWS Batch or Kubernetes. ## Introduction This document describes the TES API and provides details on the specific endpoints, request formats, and responses. It is intended to provide key information for developers of TES-compatible services as well as clients that will call these TES services. Use cases include:    - Deploying existing workflow engines on new infrastructure. Workflow engines   such as CWL-Tes and Cromwell have extentions for using TES. This will allow   a system engineer to deploy them onto a new infrastructure using a job scheduling   system not previously supported by the engine.    - Developing a custom workflow management system. This API provides a common   interface to asynchronous batch processing capabilities. A developer can write   new tools against this interface and expect them to work using a variety of   backend solutions that all support the same specification.   ## Standards The TES API specification is written in OpenAPI and embodies a RESTful service philosophy. It uses JSON in requests and responses and standard HTTP/HTTPS for information transport. HTTPS should be used rather than plain HTTP except for testing or internal-only purposes. ### Authentication and Authorization Is is envisaged that most TES API instances will require users to authenticate to use the endpoints. However, the decision if authentication is required should be taken by TES API implementers.  If authentication is required, we recommend that TES implementations use an OAuth2  bearer token, although they can choose other mechanisms if appropriate.  Checking that a user is authorized to submit TES requests is a responsibility of TES implementations. ### CORS If TES API implementation is to be used by another website or domain it must implement Cross Origin Resource Sharing (CORS). Please refer to https://w3id.org/ga4gh/product-approval-support/cors for more information about GA4GH’s recommendations and how to implement CORS. 
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`cancel_task`]
#[derive(Clone, Debug)]
pub struct CancelTaskParams {
    /// ID of task to be canceled.
    pub id: String
}

/// struct for passing parameters to the method [`create_task`]
#[derive(Clone, Debug)]
pub struct CreateTaskParams {
    pub body: models::TesTask
}

/// struct for passing parameters to the method [`get_task`]
#[derive(Clone, Debug)]
pub struct GetTaskParams {
    /// ID of task to retrieve.
    pub id: String,
    /// OPTIONAL. Affects the fields included in the returned Task messages.  `MINIMAL`: Task message will include ONLY the fields: - `tesTask.Id` - `tesTask.State`  `BASIC`: Task message will include all fields EXCEPT: - `tesTask.ExecutorLog.stdout` - `tesTask.ExecutorLog.stderr` - `tesInput.content` - `tesTaskLog.system_logs`  `FULL`: Task message includes all fields.
    pub view: Option<String>
}

/// struct for passing parameters to the method [`list_tasks`]
#[derive(Clone, Debug)]
pub struct ListTasksParams {
    /// OPTIONAL. Filter the list to include tasks where the name matches this prefix. If unspecified, no task name filtering is done.
    pub name_prefix: Option<String>,
    /// OPTIONAL. Filter tasks by state. If unspecified, no task state filtering is done.
    pub state: Option<models::TesState>,
    /// OPTIONAL. Provide key tag to filter. The field tag_key is an array of key values, and will be zipped with an optional tag_value array. So the query: ```   ?tag_key=foo1&tag_value=bar1&tag_key=foo2&tag_value=bar2 ``` Should be constructed into the structure { \"foo1\" : \"bar1\", \"foo2\" : \"bar2\"}  ```   ?tag_key=foo1 ``` Should be constructed into the structure {\"foo1\" : \"\"}  If the tag_value is empty, it will be treated as matching any possible value. If a tag value is provided, both the tag's key and value must be exact matches for a task to be returned. Filter                            Tags                          Match? ---------------------------------------------------------------------- {\"foo\": \"bar\"}                    {\"foo\": \"bar\"}                Yes {\"foo\": \"bar\"}                    {\"foo\": \"bat\"}                No {\"foo\": \"\"}                       {\"foo\": \"\"}                   Yes {\"foo\": \"bar\", \"baz\": \"bat\"}      {\"foo\": \"bar\", \"baz\": \"bat\"}  Yes {\"foo\": \"bar\"}                    {\"foo\": \"bar\", \"baz\": \"bat\"}  Yes {\"foo\": \"bar\", \"baz\": \"bat\"}      {\"foo\": \"bar\"}                No {\"foo\": \"\"}                       {\"foo\": \"bar\"}                Yes {\"foo\": \"\"}                       {}                            No
    pub tag_key: Option<Vec<String>>,
    /// OPTIONAL. The companion value field for tag_key
    pub tag_value: Option<Vec<String>>,
    /// Optional number of tasks to return in one page. Must be less than 2048. Defaults to 256.
    pub page_size: Option<i32>,
    /// OPTIONAL. Page token is used to retrieve the next page of results. If unspecified, returns the first page of results. The value can be found in the `next_page_token` field of the last returned result of ListTasks
    pub page_token: Option<String>,
    /// OPTIONAL. Affects the fields included in the returned Task messages.  `MINIMAL`: Task message will include ONLY the fields: - `tesTask.Id` - `tesTask.State`  `BASIC`: Task message will include all fields EXCEPT: - `tesTask.ExecutorLog.stdout` - `tesTask.ExecutorLog.stderr` - `tesInput.content` - `tesTaskLog.system_logs`  `FULL`: Task message includes all fields.
    pub view: Option<String>
}


/// struct for typed errors of method [`cancel_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelTaskError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTaskError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_service_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceInfoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTaskError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tasks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTasksError {
    UnknownValue(serde_json::Value),
}


/// Cancel a task based on providing an exact task ID.
pub async fn cancel_task(configuration: &configuration::Configuration, params: CancelTaskParams) -> Result<serde_json::Value, Error<CancelTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks/{id}:cancel", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CancelTaskError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new task. The user provides a Task document, which the server uses as a basis and adds additional fields.
pub async fn create_task(configuration: &configuration::Configuration, params: CreateTaskParams) -> Result<models::TesCreateTaskResponse, Error<CreateTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let body = params.body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTaskError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Provides information about the service, this structure is based on the standardized GA4GH service info structure. In addition, this endpoint will also provide information about customized storage endpoints offered by the TES server.
pub async fn get_service_info(configuration: &configuration::Configuration) -> Result<models::TesServiceInfo, Error<GetServiceInfoError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service-info", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetServiceInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a single task, based on providing the exact task ID string.
pub async fn get_task(configuration: &configuration::Configuration, params: GetTaskParams) -> Result<models::TesTask, Error<GetTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let view = params.view;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = view {
        local_var_req_builder = local_var_req_builder.query(&[("view", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTaskError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List tasks tracked by the TES server. This includes queued, active and completed tasks. How long completed tasks are stored by the system may be dependent on the underlying implementation.
pub async fn list_tasks(configuration: &configuration::Configuration, params: ListTasksParams) -> Result<models::TesListTasksResponse, Error<ListTasksError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name_prefix = params.name_prefix;
    let state = params.state;
    let tag_key = params.tag_key;
    let tag_value = params.tag_value;
    let page_size = params.page_size;
    let page_token = params.page_token;
    let view = params.view;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name_prefix {
        local_var_req_builder = local_var_req_builder.query(&[("name_prefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = state {
        local_var_req_builder = local_var_req_builder.query(&[("state", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tag_key {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("tag_key".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("tag_key", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = tag_value {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("tag_value".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("tag_value", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("page_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = view {
        local_var_req_builder = local_var_req_builder.query(&[("view", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListTasksError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

