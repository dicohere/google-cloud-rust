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

/// Implements a client for the Storage Control API.
#[derive(Clone, Debug)]
pub struct StorageControl {
    inner: std::sync::Arc<dyn super::stub::dynamic::StorageControl>,
}

impl StorageControl {

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::StorageControl + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::StorageControl>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::StorageControl> {
        super::transport::StorageControl::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::StorageControl> {
        Self::build_transport(conf).await.map(super::tracing::StorageControl::new)
    }

    /// Creates a new folder. This operation is only applicable to a hierarchical
    /// namespace enabled bucket.
    pub fn create_folder(&self) -> super::builder::storage_control::CreateFolder
    {
        super::builder::storage_control::CreateFolder::new(self.inner.clone())
    }

    /// Permanently deletes an empty folder. This operation is only applicable to a
    /// hierarchical namespace enabled bucket.
    pub fn delete_folder(&self) -> super::builder::storage_control::DeleteFolder
    {
        super::builder::storage_control::DeleteFolder::new(self.inner.clone())
    }

    /// Returns metadata for the specified folder. This operation is only
    /// applicable to a hierarchical namespace enabled bucket.
    pub fn get_folder(&self) -> super::builder::storage_control::GetFolder
    {
        super::builder::storage_control::GetFolder::new(self.inner.clone())
    }

    /// Retrieves a list of folders. This operation is only applicable to a
    /// hierarchical namespace enabled bucket.
    pub fn list_folders(&self) -> super::builder::storage_control::ListFolders
    {
        super::builder::storage_control::ListFolders::new(self.inner.clone())
    }

    /// Renames a source folder to a destination folder. This operation is only
    /// applicable to a hierarchical namespace enabled bucket. During a rename, the
    /// source and destination folders are locked until the long running operation
    /// completes.
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
    pub fn rename_folder(&self) -> super::builder::storage_control::RenameFolder
    {
        super::builder::storage_control::RenameFolder::new(self.inner.clone())
    }

    /// Returns the storage layout configuration for a given bucket.
    pub fn get_storage_layout(&self) -> super::builder::storage_control::GetStorageLayout
    {
        super::builder::storage_control::GetStorageLayout::new(self.inner.clone())
    }

    /// Creates a new managed folder.
    pub fn create_managed_folder(&self) -> super::builder::storage_control::CreateManagedFolder
    {
        super::builder::storage_control::CreateManagedFolder::new(self.inner.clone())
    }

    /// Permanently deletes an empty managed folder.
    pub fn delete_managed_folder(&self) -> super::builder::storage_control::DeleteManagedFolder
    {
        super::builder::storage_control::DeleteManagedFolder::new(self.inner.clone())
    }

    /// Returns metadata for the specified managed folder.
    pub fn get_managed_folder(&self) -> super::builder::storage_control::GetManagedFolder
    {
        super::builder::storage_control::GetManagedFolder::new(self.inner.clone())
    }

    /// Retrieves a list of managed folders for a given bucket.
    pub fn list_managed_folders(&self) -> super::builder::storage_control::ListManagedFolders
    {
        super::builder::storage_control::ListManagedFolders::new(self.inner.clone())
    }

    /// Creates an Anywhere Cache instance.
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
    pub fn create_anywhere_cache(&self) -> super::builder::storage_control::CreateAnywhereCache
    {
        super::builder::storage_control::CreateAnywhereCache::new(self.inner.clone())
    }

    /// Updates an Anywhere Cache instance. Mutable fields include `ttl` and
    /// `admission_policy`.
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
    pub fn update_anywhere_cache(&self) -> super::builder::storage_control::UpdateAnywhereCache
    {
        super::builder::storage_control::UpdateAnywhereCache::new(self.inner.clone())
    }

    /// Disables an Anywhere Cache instance. A disabled instance is read-only. The
    /// disablement could be revoked by calling ResumeAnywhereCache. The cache
    /// instance will be deleted automatically if it remains in the disabled state
    /// for at least one hour.
    pub fn disable_anywhere_cache(&self) -> super::builder::storage_control::DisableAnywhereCache
    {
        super::builder::storage_control::DisableAnywhereCache::new(self.inner.clone())
    }

    /// Pauses an Anywhere Cache instance.
    pub fn pause_anywhere_cache(&self) -> super::builder::storage_control::PauseAnywhereCache
    {
        super::builder::storage_control::PauseAnywhereCache::new(self.inner.clone())
    }

    /// Resumes a disabled or paused Anywhere Cache instance.
    pub fn resume_anywhere_cache(&self) -> super::builder::storage_control::ResumeAnywhereCache
    {
        super::builder::storage_control::ResumeAnywhereCache::new(self.inner.clone())
    }

    /// Gets an Anywhere Cache instance.
    pub fn get_anywhere_cache(&self) -> super::builder::storage_control::GetAnywhereCache
    {
        super::builder::storage_control::GetAnywhereCache::new(self.inner.clone())
    }

    /// Lists Anywhere Cache instances for a given bucket.
    pub fn list_anywhere_caches(&self) -> super::builder::storage_control::ListAnywhereCaches
    {
        super::builder::storage_control::ListAnywhereCaches::new(self.inner.clone())
    }

    /// Returns the Project scoped singleton IntelligenceConfig resource.
    pub fn get_project_intelligence_config(&self) -> super::builder::storage_control::GetProjectIntelligenceConfig
    {
        super::builder::storage_control::GetProjectIntelligenceConfig::new(self.inner.clone())
    }

    /// Updates the Project scoped singleton IntelligenceConfig resource.
    pub fn update_project_intelligence_config(&self) -> super::builder::storage_control::UpdateProjectIntelligenceConfig
    {
        super::builder::storage_control::UpdateProjectIntelligenceConfig::new(self.inner.clone())
    }

    /// Returns the Folder scoped singleton IntelligenceConfig resource.
    pub fn get_folder_intelligence_config(&self) -> super::builder::storage_control::GetFolderIntelligenceConfig
    {
        super::builder::storage_control::GetFolderIntelligenceConfig::new(self.inner.clone())
    }

    /// Updates the Folder scoped singleton IntelligenceConfig resource.
    pub fn update_folder_intelligence_config(&self) -> super::builder::storage_control::UpdateFolderIntelligenceConfig
    {
        super::builder::storage_control::UpdateFolderIntelligenceConfig::new(self.inner.clone())
    }

    /// Returns the Organization scoped singleton IntelligenceConfig resource.
    pub fn get_organization_intelligence_config(&self) -> super::builder::storage_control::GetOrganizationIntelligenceConfig
    {
        super::builder::storage_control::GetOrganizationIntelligenceConfig::new(self.inner.clone())
    }

    /// Updates the Organization scoped singleton IntelligenceConfig resource.
    pub fn update_organization_intelligence_config(&self) -> super::builder::storage_control::UpdateOrganizationIntelligenceConfig
    {
        super::builder::storage_control::UpdateOrganizationIntelligenceConfig::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::storage_control::GetOperation
    {
        super::builder::storage_control::GetOperation::new(self.inner.clone())
    }
}
