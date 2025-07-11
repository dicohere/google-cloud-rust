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

/// A dyn-compatible, crate-private version of [super::StorageControl].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait StorageControl: std::fmt::Debug + Send + Sync {
    async fn create_folder(
        &self,
        req: crate::model::CreateFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>>;

    async fn delete_folder(
        &self,
        req: crate::model::DeleteFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_folder(
        &self,
        req: crate::model::GetFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>>;

    async fn list_folders(
        &self,
        req: crate::model::ListFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFoldersResponse>>;

    async fn rename_folder(
        &self,
        req: crate::model::RenameFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_storage_layout(
        &self,
        req: crate::model::GetStorageLayoutRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StorageLayout>>;

    async fn create_managed_folder(
        &self,
        req: crate::model::CreateManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>>;

    async fn delete_managed_folder(
        &self,
        req: crate::model::DeleteManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_managed_folder(
        &self,
        req: crate::model::GetManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>>;

    async fn list_managed_folders(
        &self,
        req: crate::model::ListManagedFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListManagedFoldersResponse>>;

    async fn create_anywhere_cache(
        &self,
        req: crate::model::CreateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_anywhere_cache(
        &self,
        req: crate::model::UpdateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn disable_anywhere_cache(
        &self,
        req: crate::model::DisableAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn pause_anywhere_cache(
        &self,
        req: crate::model::PauseAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn resume_anywhere_cache(
        &self,
        req: crate::model::ResumeAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn get_anywhere_cache(
        &self,
        req: crate::model::GetAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn list_anywhere_caches(
        &self,
        req: crate::model::ListAnywhereCachesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAnywhereCachesResponse>>;

    async fn get_project_intelligence_config(
        &self,
        req: crate::model::GetProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn update_project_intelligence_config(
        &self,
        req: crate::model::UpdateProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn get_folder_intelligence_config(
        &self,
        req: crate::model::GetFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn update_folder_intelligence_config(
        &self,
        req: crate::model::UpdateFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn get_organization_intelligence_config(
        &self,
        req: crate::model::GetOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn update_organization_intelligence_config(
        &self,
        req: crate::model::UpdateOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait StorageControl: std::fmt::Debug {
    async fn create_folder(
        &self,
        req: crate::model::CreateFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>>;

    async fn delete_folder(
        &self,
        req: crate::model::DeleteFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_folder(
        &self,
        req: crate::model::GetFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>>;

    async fn list_folders(
        &self,
        req: crate::model::ListFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFoldersResponse>>;

    async fn rename_folder(
        &self,
        req: crate::model::RenameFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn get_storage_layout(
        &self,
        req: crate::model::GetStorageLayoutRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StorageLayout>>;

    async fn create_managed_folder(
        &self,
        req: crate::model::CreateManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>>;

    async fn delete_managed_folder(
        &self,
        req: crate::model::DeleteManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_managed_folder(
        &self,
        req: crate::model::GetManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>>;

    async fn list_managed_folders(
        &self,
        req: crate::model::ListManagedFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListManagedFoldersResponse>>;

    async fn create_anywhere_cache(
        &self,
        req: crate::model::CreateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_anywhere_cache(
        &self,
        req: crate::model::UpdateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn disable_anywhere_cache(
        &self,
        req: crate::model::DisableAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn pause_anywhere_cache(
        &self,
        req: crate::model::PauseAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn resume_anywhere_cache(
        &self,
        req: crate::model::ResumeAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn get_anywhere_cache(
        &self,
        req: crate::model::GetAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>>;

    async fn list_anywhere_caches(
        &self,
        req: crate::model::ListAnywhereCachesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAnywhereCachesResponse>>;

    async fn get_project_intelligence_config(
        &self,
        req: crate::model::GetProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn update_project_intelligence_config(
        &self,
        req: crate::model::UpdateProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn get_folder_intelligence_config(
        &self,
        req: crate::model::GetFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn update_folder_intelligence_config(
        &self,
        req: crate::model::UpdateFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn get_organization_intelligence_config(
        &self,
        req: crate::model::GetOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn update_organization_intelligence_config(
        &self,
        req: crate::model::UpdateOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::StorageControl] also implement [StorageControl].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::StorageControl> StorageControl for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_folder(
        &self,
        req: crate::model::CreateFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>> {
        T::create_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_folder(
        &self,
        req: crate::model::DeleteFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_folder(
        &self,
        req: crate::model::GetFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>> {
        T::get_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_folders(
        &self,
        req: crate::model::ListFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFoldersResponse>> {
        T::list_folders(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn rename_folder(
        &self,
        req: crate::model::RenameFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::rename_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_storage_layout(
        &self,
        req: crate::model::GetStorageLayoutRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StorageLayout>> {
        T::get_storage_layout(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_managed_folder(
        &self,
        req: crate::model::CreateManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>> {
        T::create_managed_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_managed_folder(
        &self,
        req: crate::model::DeleteManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_managed_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_managed_folder(
        &self,
        req: crate::model::GetManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>> {
        T::get_managed_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_managed_folders(
        &self,
        req: crate::model::ListManagedFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListManagedFoldersResponse>> {
        T::list_managed_folders(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_anywhere_cache(
        &self,
        req: crate::model::CreateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_anywhere_cache(
        &self,
        req: crate::model::UpdateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn disable_anywhere_cache(
        &self,
        req: crate::model::DisableAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::disable_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn pause_anywhere_cache(
        &self,
        req: crate::model::PauseAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::pause_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_anywhere_cache(
        &self,
        req: crate::model::ResumeAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::resume_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_anywhere_cache(
        &self,
        req: crate::model::GetAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::get_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_anywhere_caches(
        &self,
        req: crate::model::ListAnywhereCachesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAnywhereCachesResponse>> {
        T::list_anywhere_caches(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_project_intelligence_config(
        &self,
        req: crate::model::GetProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::get_project_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_project_intelligence_config(
        &self,
        req: crate::model::UpdateProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::update_project_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_folder_intelligence_config(
        &self,
        req: crate::model::GetFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::get_folder_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_folder_intelligence_config(
        &self,
        req: crate::model::UpdateFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::update_folder_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_organization_intelligence_config(
        &self,
        req: crate::model::GetOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::get_organization_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_organization_intelligence_config(
        &self,
        req: crate::model::UpdateOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::update_organization_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::StorageControl> StorageControl for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn create_folder(
        &self,
        req: crate::model::CreateFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>> {
        T::create_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_folder(
        &self,
        req: crate::model::DeleteFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_folder(
        &self,
        req: crate::model::GetFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Folder>> {
        T::get_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_folders(
        &self,
        req: crate::model::ListFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListFoldersResponse>> {
        T::list_folders(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn rename_folder(
        &self,
        req: crate::model::RenameFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::rename_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_storage_layout(
        &self,
        req: crate::model::GetStorageLayoutRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::StorageLayout>> {
        T::get_storage_layout(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_managed_folder(
        &self,
        req: crate::model::CreateManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>> {
        T::create_managed_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_managed_folder(
        &self,
        req: crate::model::DeleteManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_managed_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_managed_folder(
        &self,
        req: crate::model::GetManagedFolderRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ManagedFolder>> {
        T::get_managed_folder(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_managed_folders(
        &self,
        req: crate::model::ListManagedFoldersRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListManagedFoldersResponse>> {
        T::list_managed_folders(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_anywhere_cache(
        &self,
        req: crate::model::CreateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_anywhere_cache(
        &self,
        req: crate::model::UpdateAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn disable_anywhere_cache(
        &self,
        req: crate::model::DisableAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::disable_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn pause_anywhere_cache(
        &self,
        req: crate::model::PauseAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::pause_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn resume_anywhere_cache(
        &self,
        req: crate::model::ResumeAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::resume_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_anywhere_cache(
        &self,
        req: crate::model::GetAnywhereCacheRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::AnywhereCache>> {
        T::get_anywhere_cache(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_anywhere_caches(
        &self,
        req: crate::model::ListAnywhereCachesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAnywhereCachesResponse>> {
        T::list_anywhere_caches(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_project_intelligence_config(
        &self,
        req: crate::model::GetProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::get_project_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_project_intelligence_config(
        &self,
        req: crate::model::UpdateProjectIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::update_project_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_folder_intelligence_config(
        &self,
        req: crate::model::GetFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::get_folder_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_folder_intelligence_config(
        &self,
        req: crate::model::UpdateFolderIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::update_folder_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_organization_intelligence_config(
        &self,
        req: crate::model::GetOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::get_organization_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_organization_intelligence_config(
        &self,
        req: crate::model::UpdateOrganizationIntelligenceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::IntelligenceConfig>> {
        T::update_organization_intelligence_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
