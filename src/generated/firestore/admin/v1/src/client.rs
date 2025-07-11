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

/// Implements a client for the Cloud Firestore API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_firestore_admin_v1::client::FirestoreAdmin;
/// let client = FirestoreAdmin::builder().build().await?;
/// // use `client` to make requests to the Cloud Firestore API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// The Cloud Firestore Admin API.
///
/// This API provides several administrative services for Cloud Firestore.
///
/// Project, Database, Namespace, Collection, Collection Group, and Document are
/// used as defined in the Google Cloud Firestore API.
///
/// Operation: An Operation represents work being performed in the background.
///
/// The index service manages Cloud Firestore indexes.
///
/// Index creation is performed asynchronously.
/// An Operation resource is created for each such asynchronous operation.
/// The state of the operation (including any errors encountered)
/// may be queried via the Operation resource.
///
/// The Operations collection provides a record of actions performed for the
/// specified Project (including any Operations in progress). Operations are not
/// created directly but through calls on other collections or resources.
///
/// An Operation that is done may be deleted so that it is no longer listed as
/// part of the Operation collection. Operations are garbage collected after
/// 30 days. By default, ListOperations will only return in progress and failed
/// operations. To list completed operation, issue a ListOperations request with
/// the filter `done: true`.
///
/// Operations are created by service `FirestoreAdmin`, but are accessed via
/// service `google.longrunning.Operations`.
///
/// # Configuration
///
/// To configure `FirestoreAdmin` use the `with_*` methods in the type returned
/// by [builder()][FirestoreAdmin::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://firestore.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::firestore_admin::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::firestore_admin::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `FirestoreAdmin` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `FirestoreAdmin` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct FirestoreAdmin {
    inner: std::sync::Arc<dyn super::stub::dynamic::FirestoreAdmin>,
}

impl FirestoreAdmin {
    /// Returns a builder for [FirestoreAdmin].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_firestore_admin_v1::client::FirestoreAdmin;
    /// let client = FirestoreAdmin::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::firestore_admin::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::firestore_admin::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::FirestoreAdmin + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::FirestoreAdmin>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::FirestoreAdmin> {
        super::transport::FirestoreAdmin::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::FirestoreAdmin> {
        Self::build_transport(conf).await.map(super::tracing::FirestoreAdmin::new)
    }

    /// Creates a composite index. This returns a
    /// [google.longrunning.Operation][google.longrunning.Operation] which may be
    /// used to track the status of the creation. The metadata for the operation
    /// will be the type
    /// [IndexOperationMetadata][google.firestore.admin.v1.IndexOperationMetadata].
    ///
    /// [google.firestore.admin.v1.IndexOperationMetadata]: crate::model::IndexOperationMetadata
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    pub fn create_index(&self) -> super::builder::firestore_admin::CreateIndex
    {
        super::builder::firestore_admin::CreateIndex::new(self.inner.clone())
    }

    /// Lists composite indexes.
    pub fn list_indexes(&self) -> super::builder::firestore_admin::ListIndexes
    {
        super::builder::firestore_admin::ListIndexes::new(self.inner.clone())
    }

    /// Gets a composite index.
    pub fn get_index(&self) -> super::builder::firestore_admin::GetIndex
    {
        super::builder::firestore_admin::GetIndex::new(self.inner.clone())
    }

    /// Deletes a composite index.
    pub fn delete_index(&self) -> super::builder::firestore_admin::DeleteIndex
    {
        super::builder::firestore_admin::DeleteIndex::new(self.inner.clone())
    }

    /// Gets the metadata and configuration for a Field.
    pub fn get_field(&self) -> super::builder::firestore_admin::GetField
    {
        super::builder::firestore_admin::GetField::new(self.inner.clone())
    }

    /// Updates a field configuration. Currently, field updates apply only to
    /// single field index configuration. However, calls to
    /// [FirestoreAdmin.UpdateField][google.firestore.admin.v1.FirestoreAdmin.UpdateField]
    /// should provide a field mask to avoid changing any configuration that the
    /// caller isn't aware of. The field mask should be specified as: `{ paths:
    /// "index_config" }`.
    ///
    /// This call returns a
    /// [google.longrunning.Operation][google.longrunning.Operation] which may be
    /// used to track the status of the field update. The metadata for the
    /// operation will be the type
    /// [FieldOperationMetadata][google.firestore.admin.v1.FieldOperationMetadata].
    ///
    /// To configure the default field settings for the database, use
    /// the special `Field` with resource name:
    /// `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*`.
    ///
    /// [google.firestore.admin.v1.FieldOperationMetadata]: crate::model::FieldOperationMetadata
    /// [google.firestore.admin.v1.FirestoreAdmin.UpdateField]: crate::client::FirestoreAdmin::update_field
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    pub fn update_field(&self) -> super::builder::firestore_admin::UpdateField
    {
        super::builder::firestore_admin::UpdateField::new(self.inner.clone())
    }

    /// Lists the field configuration and metadata for this database.
    ///
    /// Currently,
    /// [FirestoreAdmin.ListFields][google.firestore.admin.v1.FirestoreAdmin.ListFields]
    /// only supports listing fields that have been explicitly overridden. To issue
    /// this query, call
    /// [FirestoreAdmin.ListFields][google.firestore.admin.v1.FirestoreAdmin.ListFields]
    /// with the filter set to `indexConfig.usesAncestorConfig:false` or
    /// `ttlConfig:*`.
    ///
    /// [google.firestore.admin.v1.FirestoreAdmin.ListFields]: crate::client::FirestoreAdmin::list_fields
    pub fn list_fields(&self) -> super::builder::firestore_admin::ListFields
    {
        super::builder::firestore_admin::ListFields::new(self.inner.clone())
    }

    /// Exports a copy of all or a subset of documents from Google Cloud Firestore
    /// to another storage system, such as Google Cloud Storage. Recent updates to
    /// documents may not be reflected in the export. The export occurs in the
    /// background and its progress can be monitored and managed via the
    /// Operation resource that is created. The output of an export may only be
    /// used once the associated operation is done. If an export operation is
    /// cancelled before completion it may leave partial data behind in Google
    /// Cloud Storage.
    ///
    /// For more details on export behavior and output format, refer to:
    /// <https://cloud.google.com/firestore/docs/manage-data/export-import>
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
    pub fn export_documents(&self) -> super::builder::firestore_admin::ExportDocuments
    {
        super::builder::firestore_admin::ExportDocuments::new(self.inner.clone())
    }

    /// Imports documents into Google Cloud Firestore. Existing documents with the
    /// same name are overwritten. The import occurs in the background and its
    /// progress can be monitored and managed via the Operation resource that is
    /// created. If an ImportDocuments operation is cancelled, it is possible
    /// that a subset of the data has already been imported to Cloud Firestore.
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
    pub fn import_documents(&self) -> super::builder::firestore_admin::ImportDocuments
    {
        super::builder::firestore_admin::ImportDocuments::new(self.inner.clone())
    }

    /// Bulk deletes a subset of documents from Google Cloud Firestore.
    /// Documents created or updated after the underlying system starts to process
    /// the request will not be deleted. The bulk delete occurs in the background
    /// and its progress can be monitored and managed via the Operation resource
    /// that is created.
    ///
    /// For more details on bulk delete behavior, refer to:
    /// <https://cloud.google.com/firestore/docs/manage-data/bulk-delete>
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
    pub fn bulk_delete_documents(&self) -> super::builder::firestore_admin::BulkDeleteDocuments
    {
        super::builder::firestore_admin::BulkDeleteDocuments::new(self.inner.clone())
    }

    /// Create a database.
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
    pub fn create_database(&self) -> super::builder::firestore_admin::CreateDatabase
    {
        super::builder::firestore_admin::CreateDatabase::new(self.inner.clone())
    }

    /// Gets information about a database.
    pub fn get_database(&self) -> super::builder::firestore_admin::GetDatabase
    {
        super::builder::firestore_admin::GetDatabase::new(self.inner.clone())
    }

    /// List all the databases in the project.
    pub fn list_databases(&self) -> super::builder::firestore_admin::ListDatabases
    {
        super::builder::firestore_admin::ListDatabases::new(self.inner.clone())
    }

    /// Updates a database.
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
    pub fn update_database(&self) -> super::builder::firestore_admin::UpdateDatabase
    {
        super::builder::firestore_admin::UpdateDatabase::new(self.inner.clone())
    }

    /// Deletes a database.
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
    pub fn delete_database(&self) -> super::builder::firestore_admin::DeleteDatabase
    {
        super::builder::firestore_admin::DeleteDatabase::new(self.inner.clone())
    }

    /// Create a user creds.
    pub fn create_user_creds(&self) -> super::builder::firestore_admin::CreateUserCreds
    {
        super::builder::firestore_admin::CreateUserCreds::new(self.inner.clone())
    }

    /// Gets a user creds resource. Note that the returned resource does not
    /// contain the secret value itself.
    pub fn get_user_creds(&self) -> super::builder::firestore_admin::GetUserCreds
    {
        super::builder::firestore_admin::GetUserCreds::new(self.inner.clone())
    }

    /// List all user creds in the database. Note that the returned resource
    /// does not contain the secret value itself.
    pub fn list_user_creds(&self) -> super::builder::firestore_admin::ListUserCreds
    {
        super::builder::firestore_admin::ListUserCreds::new(self.inner.clone())
    }

    /// Enables a user creds. No-op if the user creds are already enabled.
    pub fn enable_user_creds(&self) -> super::builder::firestore_admin::EnableUserCreds
    {
        super::builder::firestore_admin::EnableUserCreds::new(self.inner.clone())
    }

    /// Disables a user creds. No-op if the user creds are already disabled.
    pub fn disable_user_creds(&self) -> super::builder::firestore_admin::DisableUserCreds
    {
        super::builder::firestore_admin::DisableUserCreds::new(self.inner.clone())
    }

    /// Resets the password of a user creds.
    pub fn reset_user_password(&self) -> super::builder::firestore_admin::ResetUserPassword
    {
        super::builder::firestore_admin::ResetUserPassword::new(self.inner.clone())
    }

    /// Deletes a user creds.
    pub fn delete_user_creds(&self) -> super::builder::firestore_admin::DeleteUserCreds
    {
        super::builder::firestore_admin::DeleteUserCreds::new(self.inner.clone())
    }

    /// Gets information about a backup.
    pub fn get_backup(&self) -> super::builder::firestore_admin::GetBackup
    {
        super::builder::firestore_admin::GetBackup::new(self.inner.clone())
    }

    /// Lists all the backups.
    pub fn list_backups(&self) -> super::builder::firestore_admin::ListBackups
    {
        super::builder::firestore_admin::ListBackups::new(self.inner.clone())
    }

    /// Deletes a backup.
    pub fn delete_backup(&self) -> super::builder::firestore_admin::DeleteBackup
    {
        super::builder::firestore_admin::DeleteBackup::new(self.inner.clone())
    }

    /// Creates a new database by restoring from an existing backup.
    ///
    /// The new database must be in the same cloud region or multi-region location
    /// as the existing backup. This behaves similar to
    /// [FirestoreAdmin.CreateDatabase][google.firestore.admin.v1.FirestoreAdmin.CreateDatabase]
    /// except instead of creating a new empty database, a new database is created
    /// with the database type, index configuration, and documents from an existing
    /// backup.
    ///
    /// The [long-running operation][google.longrunning.Operation] can be used to
    /// track the progress of the restore, with the Operation's
    /// [metadata][google.longrunning.Operation.metadata] field type being the
    /// [RestoreDatabaseMetadata][google.firestore.admin.v1.RestoreDatabaseMetadata].
    /// The [response][google.longrunning.Operation.response] type is the
    /// [Database][google.firestore.admin.v1.Database] if the restore was
    /// successful. The new database is not readable or writeable until the LRO has
    /// completed.
    ///
    /// [google.firestore.admin.v1.Database]: crate::model::Database
    /// [google.firestore.admin.v1.FirestoreAdmin.CreateDatabase]: crate::client::FirestoreAdmin::create_database
    /// [google.firestore.admin.v1.RestoreDatabaseMetadata]: crate::model::RestoreDatabaseMetadata
    /// [google.longrunning.Operation]: longrunning::model::Operation
    /// [google.longrunning.Operation.metadata]: longrunning::model::Operation::metadata
    /// [google.longrunning.Operation.response]: longrunning::model::Operation::result
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
    pub fn restore_database(&self) -> super::builder::firestore_admin::RestoreDatabase
    {
        super::builder::firestore_admin::RestoreDatabase::new(self.inner.clone())
    }

    /// Creates a backup schedule on a database.
    /// At most two backup schedules can be configured on a database, one daily
    /// backup schedule and one weekly backup schedule.
    pub fn create_backup_schedule(&self) -> super::builder::firestore_admin::CreateBackupSchedule
    {
        super::builder::firestore_admin::CreateBackupSchedule::new(self.inner.clone())
    }

    /// Gets information about a backup schedule.
    pub fn get_backup_schedule(&self) -> super::builder::firestore_admin::GetBackupSchedule
    {
        super::builder::firestore_admin::GetBackupSchedule::new(self.inner.clone())
    }

    /// List backup schedules.
    pub fn list_backup_schedules(&self) -> super::builder::firestore_admin::ListBackupSchedules
    {
        super::builder::firestore_admin::ListBackupSchedules::new(self.inner.clone())
    }

    /// Updates a backup schedule.
    pub fn update_backup_schedule(&self) -> super::builder::firestore_admin::UpdateBackupSchedule
    {
        super::builder::firestore_admin::UpdateBackupSchedule::new(self.inner.clone())
    }

    /// Deletes a backup schedule.
    pub fn delete_backup_schedule(&self) -> super::builder::firestore_admin::DeleteBackupSchedule
    {
        super::builder::firestore_admin::DeleteBackupSchedule::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::firestore_admin::ListOperations
    {
        super::builder::firestore_admin::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::firestore_admin::GetOperation
    {
        super::builder::firestore_admin::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::firestore_admin::DeleteOperation
    {
        super::builder::firestore_admin::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::firestore_admin::CancelOperation
    {
        super::builder::firestore_admin::CancelOperation::new(self.inner.clone())
    }
}
