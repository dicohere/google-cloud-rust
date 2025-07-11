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

/// Implements a client for the Cloud Spanner API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_spanner_admin_database_v1::client::DatabaseAdmin;
/// let client = DatabaseAdmin::builder().build().await?;
/// // use `client` to make requests to the Cloud Spanner API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Cloud Spanner Database Admin API
///
/// The Cloud Spanner Database Admin API can be used to:
///
/// * create, drop, and list databases
/// * update the schema of pre-existing databases
/// * create, delete, copy and list backups for a database
/// * restore a database from an existing backup
///
/// # Configuration
///
/// To configure `DatabaseAdmin` use the `with_*` methods in the type returned
/// by [builder()][DatabaseAdmin::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://spanner.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::database_admin::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::database_admin::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `DatabaseAdmin` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DatabaseAdmin` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct DatabaseAdmin {
    inner: std::sync::Arc<dyn super::stub::dynamic::DatabaseAdmin>,
}

impl DatabaseAdmin {
    /// Returns a builder for [DatabaseAdmin].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_spanner_admin_database_v1::client::DatabaseAdmin;
    /// let client = DatabaseAdmin::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::database_admin::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::database_admin::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::DatabaseAdmin + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::DatabaseAdmin>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::DatabaseAdmin> {
        super::transport::DatabaseAdmin::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::DatabaseAdmin> {
        Self::build_transport(conf).await.map(super::tracing::DatabaseAdmin::new)
    }

    /// Lists Cloud Spanner databases.
    pub fn list_databases(&self) -> super::builder::database_admin::ListDatabases
    {
        super::builder::database_admin::ListDatabases::new(self.inner.clone())
    }

    /// Creates a new Cloud Spanner database and starts to prepare it for serving.
    /// The returned [long-running operation][google.longrunning.Operation] will
    /// have a name of the format `<database_name>/operations/<operation_id>` and
    /// can be used to track preparation of the database. The
    /// [metadata][google.longrunning.Operation.metadata] field type is
    /// [CreateDatabaseMetadata][google.spanner.admin.database.v1.CreateDatabaseMetadata].
    /// The [response][google.longrunning.Operation.response] field type is
    /// [Database][google.spanner.admin.database.v1.Database], if successful.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    /// [google.spanner.admin.database.v1.CreateDatabaseMetadata]: crate::model::CreateDatabaseMetadata
    /// [google.spanner.admin.database.v1.Database]: crate::model::Database
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
    pub fn create_database(&self) -> super::builder::database_admin::CreateDatabase
    {
        super::builder::database_admin::CreateDatabase::new(self.inner.clone())
    }

    /// Gets the state of a Cloud Spanner database.
    pub fn get_database(&self) -> super::builder::database_admin::GetDatabase
    {
        super::builder::database_admin::GetDatabase::new(self.inner.clone())
    }

    /// Updates a Cloud Spanner database. The returned
    /// [long-running operation][google.longrunning.Operation] can be used to track
    /// the progress of updating the database. If the named database does not
    /// exist, returns `NOT_FOUND`.
    ///
    /// While the operation is pending:
    ///
    /// * The database's
    ///   [reconciling][google.spanner.admin.database.v1.Database.reconciling]
    ///   field is set to true.
    /// * Cancelling the operation is best-effort. If the cancellation succeeds,
    ///   the operation metadata's
    ///   [cancel_time][google.spanner.admin.database.v1.UpdateDatabaseMetadata.cancel_time]
    ///   is set, the updates are reverted, and the operation terminates with a
    ///   `CANCELLED` status.
    /// * New UpdateDatabase requests will return a `FAILED_PRECONDITION` error
    ///   until the pending operation is done (returns successfully or with
    ///   error).
    /// * Reading the database via the API continues to give the pre-request
    ///   values.
    ///
    /// Upon completion of the returned operation:
    ///
    /// * The new values are in effect and readable via the API.
    /// * The database's
    ///   [reconciling][google.spanner.admin.database.v1.Database.reconciling]
    ///   field becomes false.
    ///
    /// The returned [long-running operation][google.longrunning.Operation] will
    /// have a name of the format
    /// `projects/<project>/instances/<instance>/databases/<database>/operations/<operation_id>`
    /// and can be used to track the database modification. The
    /// [metadata][google.longrunning.Operation.metadata] field type is
    /// [UpdateDatabaseMetadata][google.spanner.admin.database.v1.UpdateDatabaseMetadata].
    /// The [response][google.longrunning.Operation.response] field type is
    /// [Database][google.spanner.admin.database.v1.Database], if successful.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    /// [google.spanner.admin.database.v1.Database]: crate::model::Database
    /// [google.spanner.admin.database.v1.Database.reconciling]: crate::model::Database::reconciling
    /// [google.spanner.admin.database.v1.UpdateDatabaseMetadata]: crate::model::UpdateDatabaseMetadata
    /// [google.spanner.admin.database.v1.UpdateDatabaseMetadata.cancel_time]: crate::model::UpdateDatabaseMetadata::cancel_time
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
    pub fn update_database(&self) -> super::builder::database_admin::UpdateDatabase
    {
        super::builder::database_admin::UpdateDatabase::new(self.inner.clone())
    }

    /// Updates the schema of a Cloud Spanner database by
    /// creating/altering/dropping tables, columns, indexes, etc. The returned
    /// [long-running operation][google.longrunning.Operation] will have a name of
    /// the format `<database_name>/operations/<operation_id>` and can be used to
    /// track execution of the schema change(s). The
    /// [metadata][google.longrunning.Operation.metadata] field type is
    /// [UpdateDatabaseDdlMetadata][google.spanner.admin.database.v1.UpdateDatabaseDdlMetadata].
    /// The operation has no response.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.spanner.admin.database.v1.UpdateDatabaseDdlMetadata]: crate::model::UpdateDatabaseDdlMetadata
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
    pub fn update_database_ddl(&self) -> super::builder::database_admin::UpdateDatabaseDdl
    {
        super::builder::database_admin::UpdateDatabaseDdl::new(self.inner.clone())
    }

    /// Drops (aka deletes) a Cloud Spanner database.
    /// Completed backups for the database will be retained according to their
    /// `expire_time`.
    /// Note: Cloud Spanner might continue to accept requests for a few seconds
    /// after the database has been deleted.
    pub fn drop_database(&self) -> super::builder::database_admin::DropDatabase
    {
        super::builder::database_admin::DropDatabase::new(self.inner.clone())
    }

    /// Returns the schema of a Cloud Spanner database as a list of formatted
    /// DDL statements. This method does not show pending schema updates, those may
    /// be queried using the [Operations][google.longrunning.Operations] API.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_database_ddl(&self) -> super::builder::database_admin::GetDatabaseDdl
    {
        super::builder::database_admin::GetDatabaseDdl::new(self.inner.clone())
    }

    /// Sets the access control policy on a database or backup resource.
    /// Replaces any existing policy.
    ///
    /// Authorization requires `spanner.databases.setIamPolicy`
    /// permission on [resource][google.iam.v1.SetIamPolicyRequest.resource].
    /// For backups, authorization requires `spanner.backups.setIamPolicy`
    /// permission on [resource][google.iam.v1.SetIamPolicyRequest.resource].
    ///
    /// [google.iam.v1.SetIamPolicyRequest.resource]: iam_v1::model::SetIamPolicyRequest::resource
    pub fn set_iam_policy(&self) -> super::builder::database_admin::SetIamPolicy
    {
        super::builder::database_admin::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a database or backup resource.
    /// Returns an empty policy if a database or backup exists but does not have a
    /// policy set.
    ///
    /// Authorization requires `spanner.databases.getIamPolicy` permission on
    /// [resource][google.iam.v1.GetIamPolicyRequest.resource].
    /// For backups, authorization requires `spanner.backups.getIamPolicy`
    /// permission on [resource][google.iam.v1.GetIamPolicyRequest.resource].
    ///
    /// [google.iam.v1.GetIamPolicyRequest.resource]: iam_v1::model::GetIamPolicyRequest::resource
    pub fn get_iam_policy(&self) -> super::builder::database_admin::GetIamPolicy
    {
        super::builder::database_admin::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that the caller has on the specified database or backup
    /// resource.
    ///
    /// Attempting this RPC on a non-existent Cloud Spanner database will
    /// result in a NOT_FOUND error if the user has
    /// `spanner.databases.list` permission on the containing Cloud
    /// Spanner instance. Otherwise returns an empty set of permissions.
    /// Calling this method on a backup that does not exist will
    /// result in a NOT_FOUND error if the user has
    /// `spanner.backups.list` permission on the containing instance.
    pub fn test_iam_permissions(&self) -> super::builder::database_admin::TestIamPermissions
    {
        super::builder::database_admin::TestIamPermissions::new(self.inner.clone())
    }

    /// Starts creating a new Cloud Spanner Backup.
    /// The returned backup [long-running operation][google.longrunning.Operation]
    /// will have a name of the format
    /// `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation_id>`
    /// and can be used to track creation of the backup. The
    /// [metadata][google.longrunning.Operation.metadata] field type is
    /// [CreateBackupMetadata][google.spanner.admin.database.v1.CreateBackupMetadata].
    /// The [response][google.longrunning.Operation.response] field type is
    /// [Backup][google.spanner.admin.database.v1.Backup], if successful.
    /// Cancelling the returned operation will stop the creation and delete the
    /// backup. There can be only one pending backup creation per database. Backup
    /// creation of different databases can run concurrently.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    /// [google.spanner.admin.database.v1.Backup]: crate::model::Backup
    /// [google.spanner.admin.database.v1.CreateBackupMetadata]: crate::model::CreateBackupMetadata
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
    pub fn create_backup(&self) -> super::builder::database_admin::CreateBackup
    {
        super::builder::database_admin::CreateBackup::new(self.inner.clone())
    }

    /// Starts copying a Cloud Spanner Backup.
    /// The returned backup [long-running operation][google.longrunning.Operation]
    /// will have a name of the format
    /// `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation_id>`
    /// and can be used to track copying of the backup. The operation is associated
    /// with the destination backup.
    /// The [metadata][google.longrunning.Operation.metadata] field type is
    /// [CopyBackupMetadata][google.spanner.admin.database.v1.CopyBackupMetadata].
    /// The [response][google.longrunning.Operation.response] field type is
    /// [Backup][google.spanner.admin.database.v1.Backup], if successful.
    /// Cancelling the returned operation will stop the copying and delete the
    /// destination backup. Concurrent CopyBackup requests can run on the same
    /// source backup.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    /// [google.spanner.admin.database.v1.Backup]: crate::model::Backup
    /// [google.spanner.admin.database.v1.CopyBackupMetadata]: crate::model::CopyBackupMetadata
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
    pub fn copy_backup(&self) -> super::builder::database_admin::CopyBackup
    {
        super::builder::database_admin::CopyBackup::new(self.inner.clone())
    }

    /// Gets metadata on a pending or completed
    /// [Backup][google.spanner.admin.database.v1.Backup].
    ///
    /// [google.spanner.admin.database.v1.Backup]: crate::model::Backup
    pub fn get_backup(&self) -> super::builder::database_admin::GetBackup
    {
        super::builder::database_admin::GetBackup::new(self.inner.clone())
    }

    /// Updates a pending or completed
    /// [Backup][google.spanner.admin.database.v1.Backup].
    ///
    /// [google.spanner.admin.database.v1.Backup]: crate::model::Backup
    pub fn update_backup(&self) -> super::builder::database_admin::UpdateBackup
    {
        super::builder::database_admin::UpdateBackup::new(self.inner.clone())
    }

    /// Deletes a pending or completed
    /// [Backup][google.spanner.admin.database.v1.Backup].
    ///
    /// [google.spanner.admin.database.v1.Backup]: crate::model::Backup
    pub fn delete_backup(&self) -> super::builder::database_admin::DeleteBackup
    {
        super::builder::database_admin::DeleteBackup::new(self.inner.clone())
    }

    /// Lists completed and pending backups.
    /// Backups returned are ordered by `create_time` in descending order,
    /// starting from the most recent `create_time`.
    pub fn list_backups(&self) -> super::builder::database_admin::ListBackups
    {
        super::builder::database_admin::ListBackups::new(self.inner.clone())
    }

    /// Create a new database by restoring from a completed backup. The new
    /// database must be in the same project and in an instance with the same
    /// instance configuration as the instance containing
    /// the backup. The returned database [long-running
    /// operation][google.longrunning.Operation] has a name of the format
    /// `projects/<project>/instances/<instance>/databases/<database>/operations/<operation_id>`,
    /// and can be used to track the progress of the operation, and to cancel it.
    /// The [metadata][google.longrunning.Operation.metadata] field type is
    /// [RestoreDatabaseMetadata][google.spanner.admin.database.v1.RestoreDatabaseMetadata].
    /// The [response][google.longrunning.Operation.response] type
    /// is [Database][google.spanner.admin.database.v1.Database], if
    /// successful. Cancelling the returned operation will stop the restore and
    /// delete the database.
    /// There can be only one database being restored into an instance at a time.
    /// Once the restore operation completes, a new restore operation can be
    /// initiated, without waiting for the optimize operation associated with the
    /// first restore to complete.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
    /// [google.spanner.admin.database.v1.Database]: crate::model::Database
    /// [google.spanner.admin.database.v1.RestoreDatabaseMetadata]: crate::model::RestoreDatabaseMetadata
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
    pub fn restore_database(&self) -> super::builder::database_admin::RestoreDatabase
    {
        super::builder::database_admin::RestoreDatabase::new(self.inner.clone())
    }

    /// Lists database [longrunning-operations][google.longrunning.Operation].
    /// A database operation has a name of the form
    /// `projects/<project>/instances/<instance>/databases/<database>/operations/<operation>`.
    /// The long-running operation
    /// [metadata][google.longrunning.Operation.metadata] field type
    /// `metadata.type_url` describes the type of the metadata. Operations returned
    /// include those that have completed/failed/canceled within the last 7 days,
    /// and pending operations.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    pub fn list_database_operations(&self) -> super::builder::database_admin::ListDatabaseOperations
    {
        super::builder::database_admin::ListDatabaseOperations::new(self.inner.clone())
    }

    /// Lists the backup [long-running operations][google.longrunning.Operation] in
    /// the given instance. A backup operation has a name of the form
    /// `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation>`.
    /// The long-running operation
    /// [metadata][google.longrunning.Operation.metadata] field type
    /// `metadata.type_url` describes the type of the metadata. Operations returned
    /// include those that have completed/failed/canceled within the last 7 days,
    /// and pending operations. Operations returned are ordered by
    /// `operation.metadata.value.progress.start_time` in descending order starting
    /// from the most recently started operation.
    ///
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    pub fn list_backup_operations(&self) -> super::builder::database_admin::ListBackupOperations
    {
        super::builder::database_admin::ListBackupOperations::new(self.inner.clone())
    }

    /// Lists Cloud Spanner database roles.
    pub fn list_database_roles(&self) -> super::builder::database_admin::ListDatabaseRoles
    {
        super::builder::database_admin::ListDatabaseRoles::new(self.inner.clone())
    }

    /// Adds split points to specified tables, indexes of a database.
    pub fn add_split_points(&self) -> super::builder::database_admin::AddSplitPoints
    {
        super::builder::database_admin::AddSplitPoints::new(self.inner.clone())
    }

    /// Creates a new backup schedule.
    pub fn create_backup_schedule(&self) -> super::builder::database_admin::CreateBackupSchedule
    {
        super::builder::database_admin::CreateBackupSchedule::new(self.inner.clone())
    }

    /// Gets backup schedule for the input schedule name.
    pub fn get_backup_schedule(&self) -> super::builder::database_admin::GetBackupSchedule
    {
        super::builder::database_admin::GetBackupSchedule::new(self.inner.clone())
    }

    /// Updates a backup schedule.
    pub fn update_backup_schedule(&self) -> super::builder::database_admin::UpdateBackupSchedule
    {
        super::builder::database_admin::UpdateBackupSchedule::new(self.inner.clone())
    }

    /// Deletes a backup schedule.
    pub fn delete_backup_schedule(&self) -> super::builder::database_admin::DeleteBackupSchedule
    {
        super::builder::database_admin::DeleteBackupSchedule::new(self.inner.clone())
    }

    /// Lists all the backup schedules for the database.
    pub fn list_backup_schedules(&self) -> super::builder::database_admin::ListBackupSchedules
    {
        super::builder::database_admin::ListBackupSchedules::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::database_admin::ListOperations
    {
        super::builder::database_admin::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::database_admin::GetOperation
    {
        super::builder::database_admin::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::database_admin::DeleteOperation
    {
        super::builder::database_admin::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::database_admin::CancelOperation
    {
        super::builder::database_admin::CancelOperation::new(self.inner.clone())
    }
}
