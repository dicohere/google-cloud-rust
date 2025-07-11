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

/// Defines the trait used to implement [super::client::EdgeNetwork].
///
/// Application developers may need to implement this trait to mock
/// `client::EdgeNetwork`.  In other use-cases, application developers only
/// use `client::EdgeNetwork` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait EdgeNetwork: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::EdgeNetwork::initialize_zone].
    fn initialize_zone(
        &self,
        _req: crate::model::InitializeZoneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::InitializeZoneResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_zones].
    fn list_zones(
        &self,
        _req: crate::model::ListZonesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListZonesResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_zone].
    fn get_zone(
        &self,
        _req: crate::model::GetZoneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Zone>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_networks].
    fn list_networks(
        &self,
        _req: crate::model::ListNetworksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListNetworksResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_network].
    fn get_network(
        &self,
        _req: crate::model::GetNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Network>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::diagnose_network].
    fn diagnose_network(
        &self,
        _req: crate::model::DiagnoseNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::DiagnoseNetworkResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_network].
    fn create_network(
        &self,
        _req: crate::model::CreateNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_network].
    fn delete_network(
        &self,
        _req: crate::model::DeleteNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_subnets].
    fn list_subnets(
        &self,
        _req: crate::model::ListSubnetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSubnetsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_subnet].
    fn get_subnet(
        &self,
        _req: crate::model::GetSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Subnet>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_subnet].
    fn create_subnet(
        &self,
        _req: crate::model::CreateSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::update_subnet].
    fn update_subnet(
        &self,
        _req: crate::model::UpdateSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_subnet].
    fn delete_subnet(
        &self,
        _req: crate::model::DeleteSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_interconnects].
    fn list_interconnects(
        &self,
        _req: crate::model::ListInterconnectsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListInterconnectsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_interconnect].
    fn get_interconnect(
        &self,
        _req: crate::model::GetInterconnectRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Interconnect>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::diagnose_interconnect].
    fn diagnose_interconnect(
        &self,
        _req: crate::model::DiagnoseInterconnectRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::DiagnoseInterconnectResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_interconnect_attachments].
    fn list_interconnect_attachments(
        &self,
        _req: crate::model::ListInterconnectAttachmentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListInterconnectAttachmentsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_interconnect_attachment].
    fn get_interconnect_attachment(
        &self,
        _req: crate::model::GetInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::InterconnectAttachment>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_interconnect_attachment].
    fn create_interconnect_attachment(
        &self,
        _req: crate::model::CreateInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_interconnect_attachment].
    fn delete_interconnect_attachment(
        &self,
        _req: crate::model::DeleteInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_routers].
    fn list_routers(
        &self,
        _req: crate::model::ListRoutersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRoutersResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_router].
    fn get_router(
        &self,
        _req: crate::model::GetRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Router>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::diagnose_router].
    fn diagnose_router(
        &self,
        _req: crate::model::DiagnoseRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::DiagnoseRouterResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_router].
    fn create_router(
        &self,
        _req: crate::model::CreateRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::update_router].
    fn update_router(
        &self,
        _req: crate::model::UpdateRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_router].
    fn delete_router(
        &self,
        _req: crate::model::DeleteRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::cancel_operation].
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
pub trait EdgeNetwork: std::fmt::Debug {

    /// Implements [super::client::EdgeNetwork::initialize_zone].
    fn initialize_zone(
        &self,
        _req: crate::model::InitializeZoneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::InitializeZoneResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_zones].
    fn list_zones(
        &self,
        _req: crate::model::ListZonesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListZonesResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_zone].
    fn get_zone(
        &self,
        _req: crate::model::GetZoneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Zone>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_networks].
    fn list_networks(
        &self,
        _req: crate::model::ListNetworksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListNetworksResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_network].
    fn get_network(
        &self,
        _req: crate::model::GetNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Network>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::diagnose_network].
    fn diagnose_network(
        &self,
        _req: crate::model::DiagnoseNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::DiagnoseNetworkResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_network].
    fn create_network(
        &self,
        _req: crate::model::CreateNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_network].
    fn delete_network(
        &self,
        _req: crate::model::DeleteNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_subnets].
    fn list_subnets(
        &self,
        _req: crate::model::ListSubnetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListSubnetsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_subnet].
    fn get_subnet(
        &self,
        _req: crate::model::GetSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Subnet>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_subnet].
    fn create_subnet(
        &self,
        _req: crate::model::CreateSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::update_subnet].
    fn update_subnet(
        &self,
        _req: crate::model::UpdateSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_subnet].
    fn delete_subnet(
        &self,
        _req: crate::model::DeleteSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_interconnects].
    fn list_interconnects(
        &self,
        _req: crate::model::ListInterconnectsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListInterconnectsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_interconnect].
    fn get_interconnect(
        &self,
        _req: crate::model::GetInterconnectRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Interconnect>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::diagnose_interconnect].
    fn diagnose_interconnect(
        &self,
        _req: crate::model::DiagnoseInterconnectRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::DiagnoseInterconnectResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_interconnect_attachments].
    fn list_interconnect_attachments(
        &self,
        _req: crate::model::ListInterconnectAttachmentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListInterconnectAttachmentsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_interconnect_attachment].
    fn get_interconnect_attachment(
        &self,
        _req: crate::model::GetInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::InterconnectAttachment>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_interconnect_attachment].
    fn create_interconnect_attachment(
        &self,
        _req: crate::model::CreateInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_interconnect_attachment].
    fn delete_interconnect_attachment(
        &self,
        _req: crate::model::DeleteInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_routers].
    fn list_routers(
        &self,
        _req: crate::model::ListRoutersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRoutersResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_router].
    fn get_router(
        &self,
        _req: crate::model::GetRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Router>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::diagnose_router].
    fn diagnose_router(
        &self,
        _req: crate::model::DiagnoseRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::DiagnoseRouterResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::create_router].
    fn create_router(
        &self,
        _req: crate::model::CreateRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::update_router].
    fn update_router(
        &self,
        _req: crate::model::UpdateRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_router].
    fn delete_router(
        &self,
        _req: crate::model::DeleteRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::EdgeNetwork::cancel_operation].
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

