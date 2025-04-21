use kube::{api::ObjectMeta, CustomResource};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::BTreeMap;


#[derive(CustomResource,  Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[kube(
    group = "example.com",
    version = "v1",
    kind = "MyResource",
    namespaced
)]
#[kube(status = "MyResourceStatus")]
pub struct MyResourceSpec {
    pub size: i32,
    pub message: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>, 
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct MyResourceStatus {
    pub ready: bool,
    pub message: Option<String>,
}

