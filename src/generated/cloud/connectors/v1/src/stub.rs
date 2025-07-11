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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::Connectors].
///
/// Application developers may need to implement this trait to mock
/// `client::Connectors`.  In other use-cases, application developers only
/// use `client::Connectors` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait Connectors: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::Connectors::list_connections].
    fn list_connections(
        &self,
        _req: crate::model::ListConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connection].
    fn get_connection(
        &self,
        _req: crate::model::GetConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Connection>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::create_connection].
    fn create_connection(
        &self,
        _req: crate::model::CreateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::update_connection].
    fn update_connection(
        &self,
        _req: crate::model::UpdateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::delete_connection].
    fn delete_connection(
        &self,
        _req: crate::model::DeleteConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_providers].
    fn list_providers(
        &self,
        _req: crate::model::ListProvidersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListProvidersResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_provider].
    fn get_provider(
        &self,
        _req: crate::model::GetProviderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Provider>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_connectors].
    fn list_connectors(
        &self,
        _req: crate::model::ListConnectorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConnectorsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connector].
    fn get_connector(
        &self,
        _req: crate::model::GetConnectorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Connector>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_connector_versions].
    fn list_connector_versions(
        &self,
        _req: crate::model::ListConnectorVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConnectorVersionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connector_version].
    fn get_connector_version(
        &self,
        _req: crate::model::GetConnectorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ConnectorVersion>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connection_schema_metadata].
    fn get_connection_schema_metadata(
        &self,
        _req: crate::model::GetConnectionSchemaMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ConnectionSchemaMetadata>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::refresh_connection_schema_metadata].
    fn refresh_connection_schema_metadata(
        &self,
        _req: crate::model::RefreshConnectionSchemaMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_runtime_entity_schemas].
    fn list_runtime_entity_schemas(
        &self,
        _req: crate::model::ListRuntimeEntitySchemasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRuntimeEntitySchemasResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_runtime_action_schemas].
    fn list_runtime_action_schemas(
        &self,
        _req: crate::model::ListRuntimeActionSchemasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRuntimeActionSchemasResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_runtime_config].
    fn get_runtime_config(
        &self,
        _req: crate::model::GetRuntimeConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::RuntimeConfig>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_global_settings].
    fn get_global_settings(
        &self,
        _req: crate::model::GetGlobalSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Settings>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        std::sync::Arc::new(gax::polling_error_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        std::sync::Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait Connectors: std::fmt::Debug {

    /// Implements [super::client::Connectors::list_connections].
    fn list_connections(
        &self,
        _req: crate::model::ListConnectionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connection].
    fn get_connection(
        &self,
        _req: crate::model::GetConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Connection>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::create_connection].
    fn create_connection(
        &self,
        _req: crate::model::CreateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::update_connection].
    fn update_connection(
        &self,
        _req: crate::model::UpdateConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::delete_connection].
    fn delete_connection(
        &self,
        _req: crate::model::DeleteConnectionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_providers].
    fn list_providers(
        &self,
        _req: crate::model::ListProvidersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListProvidersResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_provider].
    fn get_provider(
        &self,
        _req: crate::model::GetProviderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Provider>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_connectors].
    fn list_connectors(
        &self,
        _req: crate::model::ListConnectorsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConnectorsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connector].
    fn get_connector(
        &self,
        _req: crate::model::GetConnectorRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Connector>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_connector_versions].
    fn list_connector_versions(
        &self,
        _req: crate::model::ListConnectorVersionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConnectorVersionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connector_version].
    fn get_connector_version(
        &self,
        _req: crate::model::GetConnectorVersionRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ConnectorVersion>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_connection_schema_metadata].
    fn get_connection_schema_metadata(
        &self,
        _req: crate::model::GetConnectionSchemaMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ConnectionSchemaMetadata>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::refresh_connection_schema_metadata].
    fn refresh_connection_schema_metadata(
        &self,
        _req: crate::model::RefreshConnectionSchemaMetadataRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_runtime_entity_schemas].
    fn list_runtime_entity_schemas(
        &self,
        _req: crate::model::ListRuntimeEntitySchemasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRuntimeEntitySchemasResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_runtime_action_schemas].
    fn list_runtime_action_schemas(
        &self,
        _req: crate::model::ListRuntimeActionSchemasRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRuntimeActionSchemasResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_runtime_config].
    fn get_runtime_config(
        &self,
        _req: crate::model::GetRuntimeConfigRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::RuntimeConfig>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_global_settings].
    fn get_global_settings(
        &self,
        _req: crate::model::GetGlobalSettingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Settings>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Connectors::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        std::sync::Arc::new(gax::polling_error_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        std::sync::Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}

