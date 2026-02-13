use kube::{Api, Client};
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    let pod: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": "rust-created-pod"
        },
        "spec": {
            "containers": [{
                "name": "nginx",
                "image": "nginx"
            }]
        }
    }))?;

    pods.create(&Default::default(), &pod).await?;

    println!("Pod created!");
    Ok(())
}
