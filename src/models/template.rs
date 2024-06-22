use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "ChangeMode", skip_serializing_if = "Option::is_none")]
    pub change_mode: Option<String>,
    #[serde(rename = "ChangeScript", skip_serializing_if = "Option::is_none")]
    pub change_script: Option<crate::models::ChangeScript>,
    #[serde(rename = "ChangeSignal", skip_serializing_if = "Option::is_none")]
    pub change_signal: Option<String>,
    #[serde(rename = "DestPath", skip_serializing_if = "Option::is_none")]
    pub dest_path: Option<String>,
    #[serde(rename = "EmbeddedTmpl", skip_serializing_if = "Option::is_none")]
    pub embedded_tmpl: Option<String>,
    #[serde(rename = "Envvars", skip_serializing_if = "Option::is_none")]
    pub envvars: Option<bool>,
    #[serde(rename = "Gid", skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
    #[serde(rename = "LeftDelim", skip_serializing_if = "Option::is_none")]
    pub left_delim: Option<String>,
    #[serde(rename = "Perms", skip_serializing_if = "Option::is_none")]
    pub perms: Option<String>,
    #[serde(rename = "RightDelim", skip_serializing_if = "Option::is_none")]
    pub right_delim: Option<String>,
    #[serde(rename = "SourcePath", skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(rename = "Splay", skip_serializing_if = "Option::is_none")]
    pub splay: Option<i64>,
    #[serde(rename = "Uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[serde(rename = "VaultGrace", skip_serializing_if = "Option::is_none")]
    pub vault_grace: Option<i64>,
    #[serde(rename = "Wait", skip_serializing_if = "Option::is_none")]
    pub wait: Option<crate::models::WaitConfig>,
}



