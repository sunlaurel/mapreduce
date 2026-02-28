mod pod_management;

use kube::{Api, Client};
use kube::api::PostParams;
use kube::Error;
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;
use crate::pod_management::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Rust Kubernetes client...");

    // Initializing constants and client
    let pod_name = "pod1";
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client.clone());

    // Testing out sequence of creating, deleting, and listening
    create(&pods, &pod_name).await?;
    create(&pods, &"pod2").await?;
    delete(&pods, &pod_name).await?;
    listen(&pods, &pod_name).await?;

    Ok(())
}
