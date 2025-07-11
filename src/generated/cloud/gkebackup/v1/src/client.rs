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

/// Implements a client for the Backup for GKE API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_gkebackup_v1::client::BackupForGKE;
/// let client = BackupForGKE::builder().build().await?;
/// // use `client` to make requests to the Backup for GKE API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// BackupForGKE allows Kubernetes administrators to configure, execute, and
/// manage backup and restore operations for their GKE clusters.
///
/// # Configuration
///
/// To configure `BackupForGKE` use the `with_*` methods in the type returned
/// by [builder()][BackupForGKE::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://gkebackup.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::backup_for_gke::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::backup_for_gke::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `BackupForGKE` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `BackupForGKE` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct BackupForGKE {
    inner: std::sync::Arc<dyn super::stub::dynamic::BackupForGKE>,
}

impl BackupForGKE {
    /// Returns a builder for [BackupForGKE].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_gkebackup_v1::client::BackupForGKE;
    /// let client = BackupForGKE::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::backup_for_gke::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::backup_for_gke::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::BackupForGKE + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::BackupForGKE>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::BackupForGKE> {
        super::transport::BackupForGKE::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::BackupForGKE> {
        Self::build_transport(conf).await.map(super::tracing::BackupForGKE::new)
    }

    /// Creates a new BackupPlan in a given location.
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
    pub fn create_backup_plan(&self) -> super::builder::backup_for_gke::CreateBackupPlan
    {
        super::builder::backup_for_gke::CreateBackupPlan::new(self.inner.clone())
    }

    /// Lists BackupPlans in a given location.
    pub fn list_backup_plans(&self) -> super::builder::backup_for_gke::ListBackupPlans
    {
        super::builder::backup_for_gke::ListBackupPlans::new(self.inner.clone())
    }

    /// Retrieve the details of a single BackupPlan.
    pub fn get_backup_plan(&self) -> super::builder::backup_for_gke::GetBackupPlan
    {
        super::builder::backup_for_gke::GetBackupPlan::new(self.inner.clone())
    }

    /// Update a BackupPlan.
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
    pub fn update_backup_plan(&self) -> super::builder::backup_for_gke::UpdateBackupPlan
    {
        super::builder::backup_for_gke::UpdateBackupPlan::new(self.inner.clone())
    }

    /// Deletes an existing BackupPlan.
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
    pub fn delete_backup_plan(&self) -> super::builder::backup_for_gke::DeleteBackupPlan
    {
        super::builder::backup_for_gke::DeleteBackupPlan::new(self.inner.clone())
    }

    /// Creates a new BackupChannel in a given location.
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
    pub fn create_backup_channel(&self) -> super::builder::backup_for_gke::CreateBackupChannel
    {
        super::builder::backup_for_gke::CreateBackupChannel::new(self.inner.clone())
    }

    /// Lists BackupChannels in a given location.
    pub fn list_backup_channels(&self) -> super::builder::backup_for_gke::ListBackupChannels
    {
        super::builder::backup_for_gke::ListBackupChannels::new(self.inner.clone())
    }

    /// Retrieve the details of a single BackupChannel.
    pub fn get_backup_channel(&self) -> super::builder::backup_for_gke::GetBackupChannel
    {
        super::builder::backup_for_gke::GetBackupChannel::new(self.inner.clone())
    }

    /// Update a BackupChannel.
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
    pub fn update_backup_channel(&self) -> super::builder::backup_for_gke::UpdateBackupChannel
    {
        super::builder::backup_for_gke::UpdateBackupChannel::new(self.inner.clone())
    }

    /// Deletes an existing BackupChannel.
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
    pub fn delete_backup_channel(&self) -> super::builder::backup_for_gke::DeleteBackupChannel
    {
        super::builder::backup_for_gke::DeleteBackupChannel::new(self.inner.clone())
    }

    /// Lists BackupPlanBindings in a given location.
    pub fn list_backup_plan_bindings(&self) -> super::builder::backup_for_gke::ListBackupPlanBindings
    {
        super::builder::backup_for_gke::ListBackupPlanBindings::new(self.inner.clone())
    }

    /// Retrieve the details of a single BackupPlanBinding.
    pub fn get_backup_plan_binding(&self) -> super::builder::backup_for_gke::GetBackupPlanBinding
    {
        super::builder::backup_for_gke::GetBackupPlanBinding::new(self.inner.clone())
    }

    /// Creates a Backup for the given BackupPlan.
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
    pub fn create_backup(&self) -> super::builder::backup_for_gke::CreateBackup
    {
        super::builder::backup_for_gke::CreateBackup::new(self.inner.clone())
    }

    /// Lists the Backups for a given BackupPlan.
    pub fn list_backups(&self) -> super::builder::backup_for_gke::ListBackups
    {
        super::builder::backup_for_gke::ListBackups::new(self.inner.clone())
    }

    /// Retrieve the details of a single Backup.
    pub fn get_backup(&self) -> super::builder::backup_for_gke::GetBackup
    {
        super::builder::backup_for_gke::GetBackup::new(self.inner.clone())
    }

    /// Update a Backup.
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
    pub fn update_backup(&self) -> super::builder::backup_for_gke::UpdateBackup
    {
        super::builder::backup_for_gke::UpdateBackup::new(self.inner.clone())
    }

    /// Deletes an existing Backup.
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
    pub fn delete_backup(&self) -> super::builder::backup_for_gke::DeleteBackup
    {
        super::builder::backup_for_gke::DeleteBackup::new(self.inner.clone())
    }

    /// Lists the VolumeBackups for a given Backup.
    pub fn list_volume_backups(&self) -> super::builder::backup_for_gke::ListVolumeBackups
    {
        super::builder::backup_for_gke::ListVolumeBackups::new(self.inner.clone())
    }

    /// Retrieve the details of a single VolumeBackup.
    pub fn get_volume_backup(&self) -> super::builder::backup_for_gke::GetVolumeBackup
    {
        super::builder::backup_for_gke::GetVolumeBackup::new(self.inner.clone())
    }

    /// Creates a new RestorePlan in a given location.
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
    pub fn create_restore_plan(&self) -> super::builder::backup_for_gke::CreateRestorePlan
    {
        super::builder::backup_for_gke::CreateRestorePlan::new(self.inner.clone())
    }

    /// Lists RestorePlans in a given location.
    pub fn list_restore_plans(&self) -> super::builder::backup_for_gke::ListRestorePlans
    {
        super::builder::backup_for_gke::ListRestorePlans::new(self.inner.clone())
    }

    /// Retrieve the details of a single RestorePlan.
    pub fn get_restore_plan(&self) -> super::builder::backup_for_gke::GetRestorePlan
    {
        super::builder::backup_for_gke::GetRestorePlan::new(self.inner.clone())
    }

    /// Update a RestorePlan.
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
    pub fn update_restore_plan(&self) -> super::builder::backup_for_gke::UpdateRestorePlan
    {
        super::builder::backup_for_gke::UpdateRestorePlan::new(self.inner.clone())
    }

    /// Deletes an existing RestorePlan.
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
    pub fn delete_restore_plan(&self) -> super::builder::backup_for_gke::DeleteRestorePlan
    {
        super::builder::backup_for_gke::DeleteRestorePlan::new(self.inner.clone())
    }

    /// Creates a new RestoreChannel in a given location.
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
    pub fn create_restore_channel(&self) -> super::builder::backup_for_gke::CreateRestoreChannel
    {
        super::builder::backup_for_gke::CreateRestoreChannel::new(self.inner.clone())
    }

    /// Lists RestoreChannels in a given location.
    pub fn list_restore_channels(&self) -> super::builder::backup_for_gke::ListRestoreChannels
    {
        super::builder::backup_for_gke::ListRestoreChannels::new(self.inner.clone())
    }

    /// Retrieve the details of a single RestoreChannel.
    pub fn get_restore_channel(&self) -> super::builder::backup_for_gke::GetRestoreChannel
    {
        super::builder::backup_for_gke::GetRestoreChannel::new(self.inner.clone())
    }

    /// Update a RestoreChannel.
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
    pub fn update_restore_channel(&self) -> super::builder::backup_for_gke::UpdateRestoreChannel
    {
        super::builder::backup_for_gke::UpdateRestoreChannel::new(self.inner.clone())
    }

    /// Deletes an existing RestoreChannel.
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
    pub fn delete_restore_channel(&self) -> super::builder::backup_for_gke::DeleteRestoreChannel
    {
        super::builder::backup_for_gke::DeleteRestoreChannel::new(self.inner.clone())
    }

    /// Lists RestorePlanBindings in a given location.
    pub fn list_restore_plan_bindings(&self) -> super::builder::backup_for_gke::ListRestorePlanBindings
    {
        super::builder::backup_for_gke::ListRestorePlanBindings::new(self.inner.clone())
    }

    /// Retrieve the details of a single RestorePlanBinding.
    pub fn get_restore_plan_binding(&self) -> super::builder::backup_for_gke::GetRestorePlanBinding
    {
        super::builder::backup_for_gke::GetRestorePlanBinding::new(self.inner.clone())
    }

    /// Creates a new Restore for the given RestorePlan.
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
    pub fn create_restore(&self) -> super::builder::backup_for_gke::CreateRestore
    {
        super::builder::backup_for_gke::CreateRestore::new(self.inner.clone())
    }

    /// Lists the Restores for a given RestorePlan.
    pub fn list_restores(&self) -> super::builder::backup_for_gke::ListRestores
    {
        super::builder::backup_for_gke::ListRestores::new(self.inner.clone())
    }

    /// Retrieves the details of a single Restore.
    pub fn get_restore(&self) -> super::builder::backup_for_gke::GetRestore
    {
        super::builder::backup_for_gke::GetRestore::new(self.inner.clone())
    }

    /// Update a Restore.
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
    pub fn update_restore(&self) -> super::builder::backup_for_gke::UpdateRestore
    {
        super::builder::backup_for_gke::UpdateRestore::new(self.inner.clone())
    }

    /// Deletes an existing Restore.
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
    pub fn delete_restore(&self) -> super::builder::backup_for_gke::DeleteRestore
    {
        super::builder::backup_for_gke::DeleteRestore::new(self.inner.clone())
    }

    /// Lists the VolumeRestores for a given Restore.
    pub fn list_volume_restores(&self) -> super::builder::backup_for_gke::ListVolumeRestores
    {
        super::builder::backup_for_gke::ListVolumeRestores::new(self.inner.clone())
    }

    /// Retrieve the details of a single VolumeRestore.
    pub fn get_volume_restore(&self) -> super::builder::backup_for_gke::GetVolumeRestore
    {
        super::builder::backup_for_gke::GetVolumeRestore::new(self.inner.clone())
    }

    /// Retrieve the link to the backupIndex.
    pub fn get_backup_index_download_url(&self) -> super::builder::backup_for_gke::GetBackupIndexDownloadUrl
    {
        super::builder::backup_for_gke::GetBackupIndexDownloadUrl::new(self.inner.clone())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::backup_for_gke::ListLocations
    {
        super::builder::backup_for_gke::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::backup_for_gke::GetLocation
    {
        super::builder::backup_for_gke::GetLocation::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(&self) -> super::builder::backup_for_gke::SetIamPolicy
    {
        super::builder::backup_for_gke::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(&self) -> super::builder::backup_for_gke::GetIamPolicy
    {
        super::builder::backup_for_gke::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::backup_for_gke::TestIamPermissions
    {
        super::builder::backup_for_gke::TestIamPermissions::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::backup_for_gke::ListOperations
    {
        super::builder::backup_for_gke::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::backup_for_gke::GetOperation
    {
        super::builder::backup_for_gke::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::backup_for_gke::DeleteOperation
    {
        super::builder::backup_for_gke::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::backup_for_gke::CancelOperation
    {
        super::builder::backup_for_gke::CancelOperation::new(self.inner.clone())
    }
}
