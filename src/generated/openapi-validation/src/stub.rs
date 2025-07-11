// Copyright 2024 Google LLC
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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::SecretManagerService].
///
/// Application developers may need to implement this trait to mock
/// `client::SecretManagerService`.  In other use-cases, application developers only
/// use `client::SecretManagerService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait SecretManagerService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::SecretManagerService::list_locations].
    fn list_locations(
        &self,
        _req: crate::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListLocationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_location].
    fn get_location(
        &self,
        _req: crate::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Location>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secrets].
    fn list_secrets(
        &self,
        _req: crate::model::ListSecretsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::create_secret].
    fn create_secret(
        &self,
        _req: crate::model::CreateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secrets_by_project_and_location].
    fn list_secrets_by_project_and_location(
        &self,
        _req: crate::model::ListSecretsByProjectAndLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::create_secret_by_project_and_location].
    fn create_secret_by_project_and_location(
        &self,
        _req: crate::model::CreateSecretByProjectAndLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::add_secret_version].
    fn add_secret_version(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::add_secret_version_by_project_and_location_and_secret].
    fn add_secret_version_by_project_and_location_and_secret(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret].
    fn get_secret(
        &self,
        _req: crate::model::GetSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::delete_secret].
    fn delete_secret(
        &self,
        _req: crate::model::DeleteSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Empty>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::update_secret].
    fn update_secret(
        &self,
        _req: crate::model::UpdateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret_by_project_and_location_and_secret].
    fn get_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::GetSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::delete_secret_by_project_and_location_and_secret].
    fn delete_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Empty>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::update_secret_by_project_and_location_and_secret].
    fn update_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secret_versions].
    fn list_secret_versions(
        &self,
        _req: crate::model::ListSecretVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretVersionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secret_versions_by_project_and_location_and_secret].
    fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        _req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretVersionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret_version].
    fn get_secret_version(
        &self,
        _req: crate::model::GetSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret_version_by_project_and_location_and_secret_and_version].
    fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::access_secret_version].
    fn access_secret_version(
        &self,
        _req: crate::model::AccessSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AccessSecretVersionResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::access_secret_version_by_project_and_location_and_secret_and_version].
    fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AccessSecretVersionResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::disable_secret_version].
    fn disable_secret_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::disable_secret_version_by_project_and_location_and_secret_and_version].
    fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::enable_secret_version].
    fn enable_secret_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::enable_secret_version_by_project_and_location_and_secret_and_version].
    fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::destroy_secret_version].
    fn destroy_secret_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::destroy_secret_version_by_project_and_location_and_secret_and_version].
    fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::set_iam_policy_by_project_and_location_and_secret].
    fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        _req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: crate::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_iam_policy_by_project_and_location_and_secret].
    fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        _req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::test_iam_permissions_by_project_and_location_and_secret].
    fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait SecretManagerService: std::fmt::Debug {

    /// Implements [super::client::SecretManagerService::list_locations].
    fn list_locations(
        &self,
        _req: crate::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListLocationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_location].
    fn get_location(
        &self,
        _req: crate::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Location>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secrets].
    fn list_secrets(
        &self,
        _req: crate::model::ListSecretsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::create_secret].
    fn create_secret(
        &self,
        _req: crate::model::CreateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secrets_by_project_and_location].
    fn list_secrets_by_project_and_location(
        &self,
        _req: crate::model::ListSecretsByProjectAndLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::create_secret_by_project_and_location].
    fn create_secret_by_project_and_location(
        &self,
        _req: crate::model::CreateSecretByProjectAndLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::add_secret_version].
    fn add_secret_version(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::add_secret_version_by_project_and_location_and_secret].
    fn add_secret_version_by_project_and_location_and_secret(
        &self,
        _req: crate::model::AddSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret].
    fn get_secret(
        &self,
        _req: crate::model::GetSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::delete_secret].
    fn delete_secret(
        &self,
        _req: crate::model::DeleteSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Empty>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::update_secret].
    fn update_secret(
        &self,
        _req: crate::model::UpdateSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret_by_project_and_location_and_secret].
    fn get_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::GetSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::delete_secret_by_project_and_location_and_secret].
    fn delete_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Empty>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::update_secret_by_project_and_location_and_secret].
    fn update_secret_by_project_and_location_and_secret(
        &self,
        _req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Secret>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secret_versions].
    fn list_secret_versions(
        &self,
        _req: crate::model::ListSecretVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretVersionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::list_secret_versions_by_project_and_location_and_secret].
    fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        _req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSecretVersionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret_version].
    fn get_secret_version(
        &self,
        _req: crate::model::GetSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_secret_version_by_project_and_location_and_secret_and_version].
    fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::access_secret_version].
    fn access_secret_version(
        &self,
        _req: crate::model::AccessSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AccessSecretVersionResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::access_secret_version_by_project_and_location_and_secret_and_version].
    fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AccessSecretVersionResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::disable_secret_version].
    fn disable_secret_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::disable_secret_version_by_project_and_location_and_secret_and_version].
    fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::DisableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::enable_secret_version].
    fn enable_secret_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::enable_secret_version_by_project_and_location_and_secret_and_version].
    fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::EnableSecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::destroy_secret_version].
    fn destroy_secret_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::destroy_secret_version_by_project_and_location_and_secret_and_version].
    fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        _req: crate::model::DestroySecretVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SecretVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::set_iam_policy_by_project_and_location_and_secret].
    fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        _req: crate::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: crate::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::get_iam_policy_by_project_and_location_and_secret].
    fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        _req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecretManagerService::test_iam_permissions_by_project_and_location_and_secret].
    fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        _req: crate::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }
}

