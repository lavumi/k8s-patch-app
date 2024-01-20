
use k8s_openapi::api::apps::v1::{Deployment};
use kube::{Api, Client, Error};
use kube::api::{Patch, PatchParams};
use serde_json::{json, Value};
use json_patch;

pub struct K8sClient{}


impl K8sClient{
    async fn get_client(&self) -> Result<Client, Error> {
        //todo add credeitnal logins
        let client = Client::try_default().await?;
        Ok(client)
    }


    pub async fn get_replicas(&self,namespace: Option<&String>, deployment_name : &String)->Result<i32, Error>{
        let client = self.get_client().await?;
        let deployments:Api<Deployment> = match namespace {
            Some(ns)=>Api::namespaced(client, ns),
            None => Api::default_namespaced(client)
        };
        let deployment = deployments.get(deployment_name).await?;
        if let Some(spec) = deployment.spec {
            if let Some(replicas) = spec.replicas{
                return Ok(replicas);
            }
        };
        //이거 에러 데체 어케 내뿜는거지
        return Ok(-1);
    }

    pub async fn set_replicas(&self,namespace: Option<&String>, deployment_name : &String, count : i32)->Result<(), Error>{
        let client = self.get_client().await?;
        let deployments:Api<Deployment> = match namespace {
            Some(ns)=>Api::namespaced(client, ns),
            None => Api::default_namespaced(client)
        };
        let patches = json!({
            "spec": {
                "replicas": count
            }
        });
        let params = PatchParams::apply("k9s-patch-app");
        deployments.patch(&deployment_name, &params, &Patch::Merge(patches)).await?;
        Ok(())
    }

    pub async fn patch_params(&self, namespace: Option<&String>, deployment_name : &String, patches :json_patch::Patch)-> Result<(), Error>{
        let client = self.get_client().await?;
        let deployments:Api<Deployment> = match namespace {
            Some(ns)=>Api::namespaced(client, ns),
            None => Api::default_namespaced(client)
        };
        let params = PatchParams::apply("k9s-patch-app");
        deployments.patch(&deployment_name, &params, &Patch::<Value>::Json(patches)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kube::Error;
    use super::*;

    #[tokio::test]
    async fn test_get(){
        let c = K8sClient{};
        let deployment =  "my-webapp".to_string();
        let res = c.get_replicas(None, &deployment ).await.expect("get replicas fail");
        print!("success  count:{}" , res)
    }

    #[tokio::test]
    async fn test_set(){
        let c = K8sClient{};
        let deployment =  "my-webapp".to_string();
        c.set_replicas(None, &deployment, 3).await.expect("set test fail");
    }

    #[tokio::test]
    async fn test_set_unknown_deployment(){
        let c = K8sClient{};
        let deployment =  "my-webapp2".to_string();
        let res = c.set_replicas(None, &deployment, 3).await;
        if let Err(e_) = res {
            print!("my-webapp2 not exist - success");
        }
        else {
            panic!("my-webapp2 not exist - fail");
        }
    }


    #[tokio::test]
    async fn test_patch(){
        let ns = "default".to_string();
        let namespace = Some(&ns);
        let deployment =  "my-webapp".to_string();
        let patches = r#"[{"op": "replace", "path": "/spec/replicas", "value": 2}]"#;
        let patches :json_patch::Patch= match serde_json::from_str(&patches) {
            Ok(p) => p,
            Err(e) => {
                panic!("{}", e)
            }
        };
        let c = K8sClient{};
        c.patch_params(namespace, &deployment, patches).await.expect("TODO: panic message");
    }
}