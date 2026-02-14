use kube::{Api, Client};
use kube::api::PostParams;
use kube::Error;
use k8s_openapi::api::core::v1::Pod;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Rust Kubernetes client...");

    // Automatically uses in-cluster config if running inside K8s
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

    match pods.create(&PostParams::default(), &pod).await {
        Ok(_) => println!("Pod created successfully!"),
        Err(Error::Api(e)) if e.code == 409 => {
            println!("Pod already exists, continuing...");
        }
        Err(e) => return Err(e.into()),
    }

    println!("Pod created successfully!");

    Ok(())
}
