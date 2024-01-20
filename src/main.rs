mod k8s_client;

use clap::{arg, command, Command, ArgMatches};
use tokio;
use crate::k8s_client::K8sClient;

fn make_commands()-> ArgMatches{
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("get")
                .about("Get the Current Number of Replicas in a Deployment")
                .arg(
                    arg!(-n --namespace <NAMESPACE> "namespace or default"))
                .arg(
                    arg!(-d --deployment <DEPLOYMENT> "target deployment name")
                        .required(true))
        )
        .subcommand(
            Command::new("set")
                .about("Set the Number of Replicas in a Deployment")
                .arg(
                    arg!(-n --namespace <NAMESPACE> "namespace or default"))
                .arg(
                    arg!(-d --deployment <DEPLOYMENT> "target deployment name")
                        .required(true))
                .arg(
                    arg!(-c --count <COUNT> "number of replicas you want")
                        .required(true))
        )
        .subcommand(
            Command::new("patch")
                .about("Patch to a Deployment")
                .arg(
                    arg!(-n --namespace <NAMESPACE> "namespace or default"))
                .arg(
                    arg!(-d --deployment <DEPLOYMENT> "target deployment name")
                        .required(true))
                .arg(
                    arg!(-p --patch <PATCH_JSON> "json to patch")
                        .required(true))
        )
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let matches = make_commands();
    let result = match matches.subcommand(){
        Some(("get",sub_matches))=>{
            let namespace = sub_matches.get_one::<String>("namespace");
            let deployment = sub_matches.get_one::<String>("deployment").unwrap();
            let k8s_client = K8sClient{};
            let replicas = k8s_client.get_replicas(namespace, deployment).await?;
            print!("replica count : {}\n" , replicas);
        }
        Some(("set",sub_matches))=>{
            let namespace = sub_matches.get_one::<String>("namespace");
            let deployment = sub_matches.get_one::<String>("deployment").unwrap();
            let count = sub_matches.get_one::<String>("count").unwrap().parse::<i32>().expect("fail to get count");
            let k8s_client = K8sClient{};
            k8s_client.set_replicas(namespace, deployment, count).await?;
        }
        Some(("patch",sub_matches))=>{
            let namespace = sub_matches.get_one::<String>("namespace");
            let deployment = sub_matches.get_one::<String>("deployment").unwrap();
            let patches_str = sub_matches.get_one::<String>("patch").unwrap();
            let patches :json_patch::Patch= match serde_json::from_str(&patches_str) {
                Ok(p) => p,
                Err(e) => {
                    panic!("{}", e)
                }
            };
            let k8s_client = K8sClient{};
            k8s_client.patch_params(namespace, deployment, patches).await.expect("patch fail");
        }
        Some(_) => {
            panic!("command needed")
        }
        None => {panic!("command needed")}
    };
    Ok(result)
}

