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

/// Defines the trait used to implement [super::client::LicenseManagementService].
///
/// Application developers may need to implement this trait to mock
/// `client::LicenseManagementService`.  In other use-cases, application developers only
/// use `client::LicenseManagementService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait LicenseManagementService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::LicenseManagementService::get_license_pool].
    fn get_license_pool(
        &self,
        _req: crate::model::GetLicensePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LicensePool>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::update_license_pool].
    fn update_license_pool(
        &self,
        _req: crate::model::UpdateLicensePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LicensePool>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::assign].
    fn assign(
        &self,
        _req: crate::model::AssignRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AssignResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::unassign].
    fn unassign(
        &self,
        _req: crate::model::UnassignRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::UnassignResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::enumerate_licensed_users].
    fn enumerate_licensed_users(
        &self,
        _req: crate::model::EnumerateLicensedUsersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::EnumerateLicensedUsersResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait LicenseManagementService: std::fmt::Debug {

    /// Implements [super::client::LicenseManagementService::get_license_pool].
    fn get_license_pool(
        &self,
        _req: crate::model::GetLicensePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LicensePool>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::update_license_pool].
    fn update_license_pool(
        &self,
        _req: crate::model::UpdateLicensePoolRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LicensePool>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::assign].
    fn assign(
        &self,
        _req: crate::model::AssignRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::AssignResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::unassign].
    fn unassign(
        &self,
        _req: crate::model::UnassignRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::UnassignResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::enumerate_licensed_users].
    fn enumerate_licensed_users(
        &self,
        _req: crate::model::EnumerateLicensedUsersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::EnumerateLicensedUsersResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::LicenseManagementService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }
}

/// Defines the trait used to implement [super::client::ConsumerProcurementService].
///
/// Application developers may need to implement this trait to mock
/// `client::ConsumerProcurementService`.  In other use-cases, application developers only
/// use `client::ConsumerProcurementService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait ConsumerProcurementService: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::ConsumerProcurementService::place_order].
    fn place_order(
        &self,
        _req: crate::model::PlaceOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::get_order].
    fn get_order(
        &self,
        _req: crate::model::GetOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Order>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::list_orders].
    fn list_orders(
        &self,
        _req: crate::model::ListOrdersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListOrdersResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::modify_order].
    fn modify_order(
        &self,
        _req: crate::model::ModifyOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::cancel_order].
    fn cancel_order(
        &self,
        _req: crate::model::CancelOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
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
pub trait ConsumerProcurementService: std::fmt::Debug {

    /// Implements [super::client::ConsumerProcurementService::place_order].
    fn place_order(
        &self,
        _req: crate::model::PlaceOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::get_order].
    fn get_order(
        &self,
        _req: crate::model::GetOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Order>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::list_orders].
    fn list_orders(
        &self,
        _req: crate::model::ListOrdersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListOrdersResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::modify_order].
    fn modify_order(
        &self,
        _req: crate::model::ModifyOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::cancel_order].
    fn cancel_order(
        &self,
        _req: crate::model::CancelOrderRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::ConsumerProcurementService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
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

