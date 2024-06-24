use reqwest::Method;

use crate::{Nomad, NomadError};

impl Nomad {
    /// https://developer.hashicorp.com/nomad/api-docs/client#read-file
    pub async fn client_read_file(
        &self,
        alloc_id: &str,
        file_path: &str,
    ) -> Result<String, NomadError> {
        let req = self.request(
            Method::GET,
            &format!("client/fs/cat/{}?path={}", alloc_id, file_path),
        );

        self.send_plain(req).await
    }
}
