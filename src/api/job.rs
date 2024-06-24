use models::{JobCreateRequest, JobCreateResponse, JobListAllocationsParams};
use reqwest::Method;

use crate::{
    models::{Allocation, Job},
    Nomad, NomadError,
};

pub mod models {
    use serde::{Deserialize, Serialize};

    use crate::models::job::Job;

    #[derive(Debug, Default, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct JobCreateRequest {
        pub enforce_index: Option<bool>,
        pub eval_priority: Option<i32>,
        pub job: Job,
        pub job_modify_index: Option<i32>,
        pub policy_override: Option<bool>,
        pub preserve_counts: Option<bool>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct JobCreateResponse {
        pub eval_create_index: Option<u64>,
        #[serde(rename = "EvalID")]
        pub eval_id: Option<String>,
        pub index: Option<i32>,
        pub job_modify_index: Option<u64>,
        pub known_leader: Option<bool>,
        pub last_contact: Option<i64>,
        pub warnings: Option<String>,
    }

    #[derive(Debug, Default, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct JobListAllocationsParams {
        pub all: Option<bool>,
    }
}

impl Nomad {
    /// https://developer.hashicorp.com/nomad/api-docs/jobs#create-job
    pub async fn job_create(
        &self,
        req: &JobCreateRequest,
    ) -> Result<JobCreateResponse, NomadError> {
        let req = self.request(Method::POST, "jobs").json(req);

        self.send::<JobCreateResponse>(req).await
    }

    /// https://developer.hashicorp.com/nomad/api-docs/jobs#read-job
    pub async fn job_read(&self, job_id: &str) -> Result<Job, NomadError> {
        let req = self.request(Method::GET, &format!("job/{}", job_id));

        self.send::<Job>(req).await
    }

    pub async fn job_list_allocations(
        &self,
        job_id: &str,
        params: &JobListAllocationsParams,
    ) -> Result<Vec<Allocation>, NomadError> {
        let req = self
            .request(Method::GET, &format!("job/{}/allocations", job_id))
            .query(&params);

        self.send::<Vec<Allocation>>(req).await
    }
}
