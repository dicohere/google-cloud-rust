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

/// Implements a client for the Storage Transfer API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_storagetransfer_v1::client::StorageTransferService;
/// let client = StorageTransferService::builder().build().await?;
/// // use `client` to make requests to the Storage Transfer API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Storage Transfer Service and its protos.
/// Transfers data between between Google Cloud Storage buckets or from a data
/// source external to Google to a Cloud Storage bucket.
///
/// # Configuration
///
/// To configure `StorageTransferService` use the `with_*` methods in the type returned
/// by [builder()][StorageTransferService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://storagetransfer.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::storage_transfer_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::storage_transfer_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `StorageTransferService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `StorageTransferService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct StorageTransferService {
    inner: std::sync::Arc<dyn super::stub::dynamic::StorageTransferService>,
}

impl StorageTransferService {
    /// Returns a builder for [StorageTransferService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_storagetransfer_v1::client::StorageTransferService;
    /// let client = StorageTransferService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::storage_transfer_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::storage_transfer_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::StorageTransferService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::StorageTransferService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::StorageTransferService> {
        super::transport::StorageTransferService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::StorageTransferService> {
        Self::build_transport(conf).await.map(super::tracing::StorageTransferService::new)
    }

    /// Returns the Google service account that is used by Storage Transfer
    /// Service to access buckets in the project where transfers
    /// run or in other projects. Each Google service account is associated
    /// with one Google Cloud project. Users
    /// should add this service account to the Google Cloud Storage bucket
    /// ACLs to grant access to Storage Transfer Service. This service
    /// account is created and owned by Storage Transfer Service and can
    /// only be used by Storage Transfer Service.
    pub fn get_google_service_account(&self) -> super::builder::storage_transfer_service::GetGoogleServiceAccount
    {
        super::builder::storage_transfer_service::GetGoogleServiceAccount::new(self.inner.clone())
    }

    /// Creates a transfer job that runs periodically.
    pub fn create_transfer_job(&self) -> super::builder::storage_transfer_service::CreateTransferJob
    {
        super::builder::storage_transfer_service::CreateTransferJob::new(self.inner.clone())
    }

    /// Updates a transfer job. Updating a job's transfer spec does not affect
    /// transfer operations that are running already.
    ///
    /// **Note:** The job's [status][google.storagetransfer.v1.TransferJob.status]
    /// field can be modified using this RPC (for example, to set a job's status to
    /// [DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED],
    /// [DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED], or
    /// [ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED]).
    ///
    /// [google.storagetransfer.v1.TransferJob.Status.DELETED]: crate::model::transfer_job::Status::Deleted
    /// [google.storagetransfer.v1.TransferJob.Status.DISABLED]: crate::model::transfer_job::Status::Disabled
    /// [google.storagetransfer.v1.TransferJob.Status.ENABLED]: crate::model::transfer_job::Status::Enabled
    /// [google.storagetransfer.v1.TransferJob.status]: crate::model::TransferJob::status
    pub fn update_transfer_job(&self) -> super::builder::storage_transfer_service::UpdateTransferJob
    {
        super::builder::storage_transfer_service::UpdateTransferJob::new(self.inner.clone())
    }

    /// Gets a transfer job.
    pub fn get_transfer_job(&self) -> super::builder::storage_transfer_service::GetTransferJob
    {
        super::builder::storage_transfer_service::GetTransferJob::new(self.inner.clone())
    }

    /// Lists transfer jobs.
    pub fn list_transfer_jobs(&self) -> super::builder::storage_transfer_service::ListTransferJobs
    {
        super::builder::storage_transfer_service::ListTransferJobs::new(self.inner.clone())
    }

    /// Pauses a transfer operation.
    pub fn pause_transfer_operation(&self) -> super::builder::storage_transfer_service::PauseTransferOperation
    {
        super::builder::storage_transfer_service::PauseTransferOperation::new(self.inner.clone())
    }

    /// Resumes a transfer operation that is paused.
    pub fn resume_transfer_operation(&self) -> super::builder::storage_transfer_service::ResumeTransferOperation
    {
        super::builder::storage_transfer_service::ResumeTransferOperation::new(self.inner.clone())
    }

    /// Starts a new operation for the specified transfer job.
    /// A `TransferJob` has a maximum of one active `TransferOperation`. If this
    /// method is called while a `TransferOperation` is active, an error is
    /// returned.
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
    pub fn run_transfer_job(&self) -> super::builder::storage_transfer_service::RunTransferJob
    {
        super::builder::storage_transfer_service::RunTransferJob::new(self.inner.clone())
    }

    /// Deletes a transfer job. Deleting a transfer job sets its status to
    /// [DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED].
    ///
    /// [google.storagetransfer.v1.TransferJob.Status.DELETED]: crate::model::transfer_job::Status::Deleted
    pub fn delete_transfer_job(&self) -> super::builder::storage_transfer_service::DeleteTransferJob
    {
        super::builder::storage_transfer_service::DeleteTransferJob::new(self.inner.clone())
    }

    /// Creates an agent pool resource.
    pub fn create_agent_pool(&self) -> super::builder::storage_transfer_service::CreateAgentPool
    {
        super::builder::storage_transfer_service::CreateAgentPool::new(self.inner.clone())
    }

    /// Updates an existing agent pool resource.
    pub fn update_agent_pool(&self) -> super::builder::storage_transfer_service::UpdateAgentPool
    {
        super::builder::storage_transfer_service::UpdateAgentPool::new(self.inner.clone())
    }

    /// Gets an agent pool.
    pub fn get_agent_pool(&self) -> super::builder::storage_transfer_service::GetAgentPool
    {
        super::builder::storage_transfer_service::GetAgentPool::new(self.inner.clone())
    }

    /// Lists agent pools.
    pub fn list_agent_pools(&self) -> super::builder::storage_transfer_service::ListAgentPools
    {
        super::builder::storage_transfer_service::ListAgentPools::new(self.inner.clone())
    }

    /// Deletes an agent pool.
    pub fn delete_agent_pool(&self) -> super::builder::storage_transfer_service::DeleteAgentPool
    {
        super::builder::storage_transfer_service::DeleteAgentPool::new(self.inner.clone())
    }

    /// Lists transfer operations. Operations are ordered by their creation
    /// time in reverse chronological order.
    pub fn list_operations(&self) -> super::builder::storage_transfer_service::ListOperations
    {
        super::builder::storage_transfer_service::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::storage_transfer_service::GetOperation
    {
        super::builder::storage_transfer_service::GetOperation::new(self.inner.clone())
    }

    /// Cancels a transfer. Use
    /// the [transferOperations.get][google.longrunning.Operations.GetOperation]
    /// method to check if the cancellation succeeded or if the operation
    /// completed despite the `cancel` request.
    ///
    /// When you cancel an operation, the currently running transfer is
    /// interrupted.  For recurring transfer jobs, the next instance of the
    /// transfer job will still run.  For example, if your job is configured
    /// to run every day at 1pm and you cancel Monday's operation at 1:05pm,
    /// Monday's transfer
    /// will stop. However, a transfer job will still be attempted on Tuesday.
    ///
    /// This applies only to currently running operations. If an operation is
    /// not currently running, `cancel` does nothing.
    ///
    /// When you cancel a job, the next job computes a delta of files and may
    /// repair any inconsistent state. For instance, if you run a job every
    /// day, and today's job found 10 new files and transferred five files
    /// before you canceled the job, tomorrow's transfer operation will
    /// compute a new delta with the five files that were not copied today
    /// plus any new files discovered tomorrow.
    pub fn cancel_operation(&self) -> super::builder::storage_transfer_service::CancelOperation
    {
        super::builder::storage_transfer_service::CancelOperation::new(self.inner.clone())
    }
}
