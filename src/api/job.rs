use reqwest::Method;

use crate::api::job::models::{JobCreateRequest, JobCreateResponse};
use crate::{Nomad, NomadError};

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
}

impl Nomad {
    /// https://developer.hashicorp.com/nomad/api-docs/jobs#create-job
    pub async fn job_create(
        &self,
        req: &JobCreateRequest,
    ) -> Result<JobCreateResponse, NomadError> {
        let req = self.request(Method::POST, "/jobs").json(req);

        self.send::<JobCreateResponse>(req).await
    }
}
