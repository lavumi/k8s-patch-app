### k8s deployment replicas adjustment tool


## Features
- Retrieve the current number of replicas in a Kubernetes Deployment
- Adjust the number of replicas in a Kubernetes Deployment to a specified count


## Prerequisites

Before using this tool, ensure you have:

- Access permissions for the Kubernetes cluster
- Configured `kubeconfig` file for the Kubernetes cluster


## Usage
- Get the Current Number of Replicas in a Deployment:
```bash
./k9s-patch-app get --namespace <namespace> --deployment <deployment> 
```

- Set the Current Number of Replicas in a Deployment:
```bash
./k9s-patch-app get --namespace <namespace> --deployment <deployment> --count <count>
```

- Set the Number of Replicas in a Deployment

```bash  
./k8s-patch-app set --namespace <namespace> --deployment <deployment> --patch <patch json>
```

for more information
```
./k8s-patch-app help
```
