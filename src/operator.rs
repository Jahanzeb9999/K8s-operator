use futures::StreamExt;
use kube::{
    api::{Api, ListParams, Patch, PatchParams},
    client::Client,
    runtime::{controller::Action, watcher, Controller},
    Resource,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::crd::{MyResource, MyResourceSpec, MyResourceStatus};
use crate::error::{Error, Result};

/// Context for the controller
pub struct Context {
    client: Client,
}

/// The reconciliation function for our CRD
pub async fn reconcile(ctx: Arc<Context>, resource: MyResource) -> Result<Action> {
    // Handle the resource name safely
    match resource.metadata.name.as_deref() {
        Some(name) => info!("Reconciling resource: {}", name),  // Borrow the value inside the Option
        None => {
            error!("Resource name is missing, cannot reconcile.");
            return Ok(Action::requeue(Duration::from_secs(30)));  // Requeue if name is missing
        }
    }

    let api: Api<MyResource> = Api::namespaced(ctx.client.clone(), "default");

    // Fetch the current state of the custom resource
    let resource_status = match api.get(&resource.metadata.name.as_deref().unwrap_or_default()).await {
        Ok(status) => status,
        Err(e) => {
            error!("Failed to get resource: {}", e);
            return Ok(Action::requeue(Duration::from_secs(30)));  // Requeue if failed to get resource
        }
    };

    // Compare current state with desired state (spec)
    if resource_status.spec.size != resource.spec.size {
        info!("Resource size mismatch, updating resource...");

        // Perform the update (patch)
        let patch = Patch::Apply(MyResourceSpec {
            size: resource.spec.size,
            message: resource.spec.message.clone(),
            annotations: resource.spec.annotations.clone(),
        });

        // Patch the resource
        match api.patch(&resource.metadata.name.as_deref().unwrap_or_default(), &PatchParams::default(), &patch).await {
            Ok(_) => {
                info!("Resource successfully updated");
            }
            Err(e) => {
                error!("Failed to update resource: {}", e);
                return Ok(Action::requeue(Duration::from_secs(30)));  // Requeue in case of failure
            }
        }
    }

    sleep(Duration::from_secs(5)).await;

    // Return the action for the controller (requeue for next reconciliation)
    Ok(Action::requeue(Duration::from_secs(60)))  // Requeue every minute
}
