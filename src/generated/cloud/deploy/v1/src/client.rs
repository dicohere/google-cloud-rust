// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

/// Implements a client for the Cloud Deploy API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_deploy_v1::client::CloudDeploy;
/// let client = CloudDeploy::builder().build().await?;
/// // use `client` to make requests to the Cloud Deploy API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// CloudDeploy service creates and manages Continuous Delivery operations
/// on Google Cloud Platform via Skaffold (<https://skaffold.dev>).
///
/// # Configuration
///
/// To configure `CloudDeploy` use the `with_*` methods in the type returned
/// by [builder()][CloudDeploy::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://clouddeploy.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_deploy::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_deploy::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudDeploy` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudDeploy` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CloudDeploy {
    inner: std::sync::Arc<dyn super::stub::dynamic::CloudDeploy>,
}

impl CloudDeploy {
    /// Returns a builder for [CloudDeploy].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_deploy_v1::client::CloudDeploy;
    /// let client = CloudDeploy::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_deploy::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::cloud_deploy::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CloudDeploy + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CloudDeploy>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudDeploy> {
        super::transport::CloudDeploy::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudDeploy> {
        Self::build_transport(conf).await.map(super::tracing::CloudDeploy::new)
    }

    /// Lists DeliveryPipelines in a given project and location.
    pub fn list_delivery_pipelines(&self) -> super::builder::cloud_deploy::ListDeliveryPipelines
    {
        super::builder::cloud_deploy::ListDeliveryPipelines::new(self.inner.clone())
    }

    /// Gets details of a single DeliveryPipeline.
    pub fn get_delivery_pipeline(&self) -> super::builder::cloud_deploy::GetDeliveryPipeline
    {
        super::builder::cloud_deploy::GetDeliveryPipeline::new(self.inner.clone())
    }

    /// Creates a new DeliveryPipeline in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_delivery_pipeline(&self) -> super::builder::cloud_deploy::CreateDeliveryPipeline
    {
        super::builder::cloud_deploy::CreateDeliveryPipeline::new(self.inner.clone())
    }

    /// Updates the parameters of a single DeliveryPipeline.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_delivery_pipeline(&self) -> super::builder::cloud_deploy::UpdateDeliveryPipeline
    {
        super::builder::cloud_deploy::UpdateDeliveryPipeline::new(self.inner.clone())
    }

    /// Deletes a single DeliveryPipeline.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_delivery_pipeline(&self) -> super::builder::cloud_deploy::DeleteDeliveryPipeline
    {
        super::builder::cloud_deploy::DeleteDeliveryPipeline::new(self.inner.clone())
    }

    /// Lists Targets in a given project and location.
    pub fn list_targets(&self) -> super::builder::cloud_deploy::ListTargets
    {
        super::builder::cloud_deploy::ListTargets::new(self.inner.clone())
    }

    /// Creates a `Rollout` to roll back the specified target.
    pub fn rollback_target(&self) -> super::builder::cloud_deploy::RollbackTarget
    {
        super::builder::cloud_deploy::RollbackTarget::new(self.inner.clone())
    }

    /// Gets details of a single Target.
    pub fn get_target(&self) -> super::builder::cloud_deploy::GetTarget
    {
        super::builder::cloud_deploy::GetTarget::new(self.inner.clone())
    }

    /// Creates a new Target in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_target(&self) -> super::builder::cloud_deploy::CreateTarget
    {
        super::builder::cloud_deploy::CreateTarget::new(self.inner.clone())
    }

    /// Updates the parameters of a single Target.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_target(&self) -> super::builder::cloud_deploy::UpdateTarget
    {
        super::builder::cloud_deploy::UpdateTarget::new(self.inner.clone())
    }

    /// Deletes a single Target.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_target(&self) -> super::builder::cloud_deploy::DeleteTarget
    {
        super::builder::cloud_deploy::DeleteTarget::new(self.inner.clone())
    }

    /// Lists CustomTargetTypes in a given project and location.
    pub fn list_custom_target_types(&self) -> super::builder::cloud_deploy::ListCustomTargetTypes
    {
        super::builder::cloud_deploy::ListCustomTargetTypes::new(self.inner.clone())
    }

    /// Gets details of a single CustomTargetType.
    pub fn get_custom_target_type(&self) -> super::builder::cloud_deploy::GetCustomTargetType
    {
        super::builder::cloud_deploy::GetCustomTargetType::new(self.inner.clone())
    }

    /// Creates a new CustomTargetType in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_custom_target_type(&self) -> super::builder::cloud_deploy::CreateCustomTargetType
    {
        super::builder::cloud_deploy::CreateCustomTargetType::new(self.inner.clone())
    }

    /// Updates a single CustomTargetType.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_custom_target_type(&self) -> super::builder::cloud_deploy::UpdateCustomTargetType
    {
        super::builder::cloud_deploy::UpdateCustomTargetType::new(self.inner.clone())
    }

    /// Deletes a single CustomTargetType.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_custom_target_type(&self) -> super::builder::cloud_deploy::DeleteCustomTargetType
    {
        super::builder::cloud_deploy::DeleteCustomTargetType::new(self.inner.clone())
    }

    /// Lists Releases in a given project and location.
    pub fn list_releases(&self) -> super::builder::cloud_deploy::ListReleases
    {
        super::builder::cloud_deploy::ListReleases::new(self.inner.clone())
    }

    /// Gets details of a single Release.
    pub fn get_release(&self) -> super::builder::cloud_deploy::GetRelease
    {
        super::builder::cloud_deploy::GetRelease::new(self.inner.clone())
    }

    /// Creates a new Release in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_release(&self) -> super::builder::cloud_deploy::CreateRelease
    {
        super::builder::cloud_deploy::CreateRelease::new(self.inner.clone())
    }

    /// Abandons a Release in the Delivery Pipeline.
    pub fn abandon_release(&self) -> super::builder::cloud_deploy::AbandonRelease
    {
        super::builder::cloud_deploy::AbandonRelease::new(self.inner.clone())
    }

    /// Creates a new DeployPolicy in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_deploy_policy(&self) -> super::builder::cloud_deploy::CreateDeployPolicy
    {
        super::builder::cloud_deploy::CreateDeployPolicy::new(self.inner.clone())
    }

    /// Updates the parameters of a single DeployPolicy.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_deploy_policy(&self) -> super::builder::cloud_deploy::UpdateDeployPolicy
    {
        super::builder::cloud_deploy::UpdateDeployPolicy::new(self.inner.clone())
    }

    /// Deletes a single DeployPolicy.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_deploy_policy(&self) -> super::builder::cloud_deploy::DeleteDeployPolicy
    {
        super::builder::cloud_deploy::DeleteDeployPolicy::new(self.inner.clone())
    }

    /// Lists DeployPolicies in a given project and location.
    pub fn list_deploy_policies(&self) -> super::builder::cloud_deploy::ListDeployPolicies
    {
        super::builder::cloud_deploy::ListDeployPolicies::new(self.inner.clone())
    }

    /// Gets details of a single DeployPolicy.
    pub fn get_deploy_policy(&self) -> super::builder::cloud_deploy::GetDeployPolicy
    {
        super::builder::cloud_deploy::GetDeployPolicy::new(self.inner.clone())
    }

    /// Approves a Rollout.
    pub fn approve_rollout(&self) -> super::builder::cloud_deploy::ApproveRollout
    {
        super::builder::cloud_deploy::ApproveRollout::new(self.inner.clone())
    }

    /// Advances a Rollout in a given project and location.
    pub fn advance_rollout(&self) -> super::builder::cloud_deploy::AdvanceRollout
    {
        super::builder::cloud_deploy::AdvanceRollout::new(self.inner.clone())
    }

    /// Cancels a Rollout in a given project and location.
    pub fn cancel_rollout(&self) -> super::builder::cloud_deploy::CancelRollout
    {
        super::builder::cloud_deploy::CancelRollout::new(self.inner.clone())
    }

    /// Lists Rollouts in a given project and location.
    pub fn list_rollouts(&self) -> super::builder::cloud_deploy::ListRollouts
    {
        super::builder::cloud_deploy::ListRollouts::new(self.inner.clone())
    }

    /// Gets details of a single Rollout.
    pub fn get_rollout(&self) -> super::builder::cloud_deploy::GetRollout
    {
        super::builder::cloud_deploy::GetRollout::new(self.inner.clone())
    }

    /// Creates a new Rollout in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_rollout(&self) -> super::builder::cloud_deploy::CreateRollout
    {
        super::builder::cloud_deploy::CreateRollout::new(self.inner.clone())
    }

    /// Ignores the specified Job in a Rollout.
    pub fn ignore_job(&self) -> super::builder::cloud_deploy::IgnoreJob
    {
        super::builder::cloud_deploy::IgnoreJob::new(self.inner.clone())
    }

    /// Retries the specified Job in a Rollout.
    pub fn retry_job(&self) -> super::builder::cloud_deploy::RetryJob
    {
        super::builder::cloud_deploy::RetryJob::new(self.inner.clone())
    }

    /// Lists JobRuns in a given project and location.
    pub fn list_job_runs(&self) -> super::builder::cloud_deploy::ListJobRuns
    {
        super::builder::cloud_deploy::ListJobRuns::new(self.inner.clone())
    }

    /// Gets details of a single JobRun.
    pub fn get_job_run(&self) -> super::builder::cloud_deploy::GetJobRun
    {
        super::builder::cloud_deploy::GetJobRun::new(self.inner.clone())
    }

    /// Terminates a Job Run in a given project and location.
    pub fn terminate_job_run(&self) -> super::builder::cloud_deploy::TerminateJobRun
    {
        super::builder::cloud_deploy::TerminateJobRun::new(self.inner.clone())
    }

    /// Gets the configuration for a location.
    pub fn get_config(&self) -> super::builder::cloud_deploy::GetConfig
    {
        super::builder::cloud_deploy::GetConfig::new(self.inner.clone())
    }

    /// Creates a new Automation in a given project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_automation(&self) -> super::builder::cloud_deploy::CreateAutomation
    {
        super::builder::cloud_deploy::CreateAutomation::new(self.inner.clone())
    }

    /// Updates the parameters of a single Automation resource.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_automation(&self) -> super::builder::cloud_deploy::UpdateAutomation
    {
        super::builder::cloud_deploy::UpdateAutomation::new(self.inner.clone())
    }

    /// Deletes a single Automation resource.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_automation(&self) -> super::builder::cloud_deploy::DeleteAutomation
    {
        super::builder::cloud_deploy::DeleteAutomation::new(self.inner.clone())
    }

    /// Gets details of a single Automation.
    pub fn get_automation(&self) -> super::builder::cloud_deploy::GetAutomation
    {
        super::builder::cloud_deploy::GetAutomation::new(self.inner.clone())
    }

    /// Lists Automations in a given project and location.
    pub fn list_automations(&self) -> super::builder::cloud_deploy::ListAutomations
    {
        super::builder::cloud_deploy::ListAutomations::new(self.inner.clone())
    }

    /// Gets details of a single AutomationRun.
    pub fn get_automation_run(&self) -> super::builder::cloud_deploy::GetAutomationRun
    {
        super::builder::cloud_deploy::GetAutomationRun::new(self.inner.clone())
    }

    /// Lists AutomationRuns in a given project and location.
    pub fn list_automation_runs(&self) -> super::builder::cloud_deploy::ListAutomationRuns
    {
        super::builder::cloud_deploy::ListAutomationRuns::new(self.inner.clone())
    }

    /// Cancels an AutomationRun. The `state` of the `AutomationRun` after
    /// cancelling is `CANCELLED`. `CancelAutomationRun` can be called on
    /// AutomationRun in the state `IN_PROGRESS` and `PENDING`; AutomationRun
    /// in a different state returns an `FAILED_PRECONDITION` error.
    pub fn cancel_automation_run(&self) -> super::builder::cloud_deploy::CancelAutomationRun
    {
        super::builder::cloud_deploy::CancelAutomationRun::new(self.inner.clone())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::cloud_deploy::ListLocations
    {
        super::builder::cloud_deploy::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::cloud_deploy::GetLocation
    {
        super::builder::cloud_deploy::GetLocation::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(&self) -> super::builder::cloud_deploy::SetIamPolicy
    {
        super::builder::cloud_deploy::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(&self) -> super::builder::cloud_deploy::GetIamPolicy
    {
        super::builder::cloud_deploy::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::cloud_deploy::TestIamPermissions
    {
        super::builder::cloud_deploy::TestIamPermissions::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::cloud_deploy::ListOperations
    {
        super::builder::cloud_deploy::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::cloud_deploy::GetOperation
    {
        super::builder::cloud_deploy::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::cloud_deploy::DeleteOperation
    {
        super::builder::cloud_deploy::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::cloud_deploy::CancelOperation
    {
        super::builder::cloud_deploy::CancelOperation::new(self.inner.clone())
    }
}
