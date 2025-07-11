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

/// Defines the trait used to implement [super::client::LicenseManager].
///
/// Application developers may need to implement this trait to mock
/// `client::LicenseManager`.  In other use-cases, application developers only
/// use `client::LicenseManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait LicenseManager: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::LicenseManager::list_configurations].
    fn list_configurations(
        &self,
        _req: crate::model::ListConfigurationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConfigurationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_configuration].
    fn get_configuration(
        &self,
        _req: crate::model::GetConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Configuration>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::create_configuration].
    fn create_configuration(
        &self,
        _req: crate::model::CreateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::update_configuration].
    fn update_configuration(
        &self,
        _req: crate::model::UpdateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::delete_configuration].
    fn delete_configuration(
        &self,
        _req: crate::model::DeleteConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListInstancesResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Instance>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::deactivate_configuration].
    fn deactivate_configuration(
        &self,
        _req: crate::model::DeactivateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::reactivate_configuration].
    fn reactivate_configuration(
        &self,
        _req: crate::model::ReactivateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::query_configuration_license_usage].
    fn query_configuration_license_usage(
        &self,
        _req: crate::model::QueryConfigurationLicenseUsageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::QueryConfigurationLicenseUsageResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::aggregate_usage].
    fn aggregate_usage(
        &self,
        _req: crate::model::AggregateUsageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AggregateUsageResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_products].
    fn list_products(
        &self,
        _req: crate::model::ListProductsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListProductsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_product].
    fn get_product(
        &self,
        _req: crate::model::GetProductRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Product>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::cancel_operation].
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
pub trait LicenseManager: std::fmt::Debug {

    /// Implements [super::client::LicenseManager::list_configurations].
    fn list_configurations(
        &self,
        _req: crate::model::ListConfigurationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListConfigurationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_configuration].
    fn get_configuration(
        &self,
        _req: crate::model::GetConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Configuration>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::create_configuration].
    fn create_configuration(
        &self,
        _req: crate::model::CreateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::update_configuration].
    fn update_configuration(
        &self,
        _req: crate::model::UpdateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::delete_configuration].
    fn delete_configuration(
        &self,
        _req: crate::model::DeleteConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListInstancesResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Instance>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::deactivate_configuration].
    fn deactivate_configuration(
        &self,
        _req: crate::model::DeactivateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::reactivate_configuration].
    fn reactivate_configuration(
        &self,
        _req: crate::model::ReactivateConfigurationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::query_configuration_license_usage].
    fn query_configuration_license_usage(
        &self,
        _req: crate::model::QueryConfigurationLicenseUsageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::QueryConfigurationLicenseUsageResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::aggregate_usage].
    fn aggregate_usage(
        &self,
        _req: crate::model::AggregateUsageRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AggregateUsageResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_products].
    fn list_products(
        &self,
        _req: crate::model::ListProductsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListProductsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_product].
    fn get_product(
        &self,
        _req: crate::model::GetProductRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Product>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManager::cancel_operation].
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

