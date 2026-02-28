use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, PostParams, LogParams}, 
    Error};
use serde_json::json;
use futures_util::io::AsyncBufReadExt;


pub async fn create(pods: &Api<Pod>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Automatically uses in-cluster config if running inside K8s
    let pod: Pod = serde_json::from_value(json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": name
        },
        "spec": {
            "containers": [{
                "name": "pod-test",
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

    println!("Done!");
    Ok(())
}

pub async fn delete(pods: &Api<Pod>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    match pods.delete(name, &DeleteParams::default()).await {
        Ok(_) => {
            println!("Deleted pod: {name}");
            Ok(())
        }
        Err(Error::Api(e)) if e.code == 404 => {
            println!("Pod not found, can't delete {name}");
            Ok(())
        }
        Err(e) => Err(e.into())
    }
}

pub async fn listen(pods: &Api<Pod>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lp = LogParams {
        follow: true,
        container: Some("pod-test".into()),
        ..LogParams::default()
    };

    let mut stream = pods.log_stream(name, &lp).await?;
    let mut line = String::new();

    loop {
        line.clear();
        let n = stream.read_line(&mut line).await?;
        if n == 0 {
            break; // EOF (or stream ended)
        }
        print!("{line}");
    }
    
    Ok(())
}

