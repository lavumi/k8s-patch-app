use clap::{arg, command, ArgAction, Command};
use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client};
use kube::api::{Patch, PatchParams};
use serde_json::{Value};
use tokio;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let matches = command!()
        .subcommand(
            Command::new("get")
                .about("")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .arg(
            arg!(-n --name <DEPLOYMENT> "target deployment name")
                .required(true))
        .arg(
            arg!(-p --patch <JSON> "k8s json patch")
            .required(true))
        .get_matches();

    let name = matches.get_one::<String>("name").cloned().unwrap();
    let patches_str = matches.get_one::<String>("patch").cloned().unwrap();

    let patches = serde_json::from_str(&patches_str).unwrap();

    let client = Client::try_default().await?;
    let deployments:Api<Deployment> = Api::default_namespaced(client);
    let params = PatchParams::apply("k9s-patch-app");
    deployments.patch(&name, &params, &Patch::<Value>::Json(patches)).await?;

    Ok(())
}