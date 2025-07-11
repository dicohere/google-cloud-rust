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

/// Defines the trait used to implement [super::client::Lineage].
///
/// Application developers may need to implement this trait to mock
/// `client::Lineage`.  In other use-cases, application developers only
/// use `client::Lineage` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait Lineage: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::Lineage::process_open_lineage_run_event].
    fn process_open_lineage_run_event(
        &self,
        _req: crate::model::ProcessOpenLineageRunEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ProcessOpenLineageRunEventResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::create_process].
    fn create_process(
        &self,
        _req: crate::model::CreateProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Process>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::update_process].
    fn update_process(
        &self,
        _req: crate::model::UpdateProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Process>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_process].
    fn get_process(
        &self,
        _req: crate::model::GetProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Process>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_processes].
    fn list_processes(
        &self,
        _req: crate::model::ListProcessesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListProcessesResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_process].
    fn delete_process(
        &self,
        _req: crate::model::DeleteProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::create_run].
    fn create_run(
        &self,
        _req: crate::model::CreateRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Run>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::update_run].
    fn update_run(
        &self,
        _req: crate::model::UpdateRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Run>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_run].
    fn get_run(
        &self,
        _req: crate::model::GetRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Run>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_runs].
    fn list_runs(
        &self,
        _req: crate::model::ListRunsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRunsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_run].
    fn delete_run(
        &self,
        _req: crate::model::DeleteRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::create_lineage_event].
    fn create_lineage_event(
        &self,
        _req: crate::model::CreateLineageEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LineageEvent>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_lineage_event].
    fn get_lineage_event(
        &self,
        _req: crate::model::GetLineageEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LineageEvent>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_lineage_events].
    fn list_lineage_events(
        &self,
        _req: crate::model::ListLineageEventsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListLineageEventsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_lineage_event].
    fn delete_lineage_event(
        &self,
        _req: crate::model::DeleteLineageEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::search_links].
    fn search_links(
        &self,
        _req: crate::model::SearchLinksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SearchLinksResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::batch_search_link_processes].
    fn batch_search_link_processes(
        &self,
        _req: crate::model::BatchSearchLinkProcessesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::BatchSearchLinkProcessesResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::cancel_operation].
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
pub trait Lineage: std::fmt::Debug {

    /// Implements [super::client::Lineage::process_open_lineage_run_event].
    fn process_open_lineage_run_event(
        &self,
        _req: crate::model::ProcessOpenLineageRunEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ProcessOpenLineageRunEventResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::create_process].
    fn create_process(
        &self,
        _req: crate::model::CreateProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Process>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::update_process].
    fn update_process(
        &self,
        _req: crate::model::UpdateProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Process>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_process].
    fn get_process(
        &self,
        _req: crate::model::GetProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Process>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_processes].
    fn list_processes(
        &self,
        _req: crate::model::ListProcessesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListProcessesResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_process].
    fn delete_process(
        &self,
        _req: crate::model::DeleteProcessRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::create_run].
    fn create_run(
        &self,
        _req: crate::model::CreateRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Run>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::update_run].
    fn update_run(
        &self,
        _req: crate::model::UpdateRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Run>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_run].
    fn get_run(
        &self,
        _req: crate::model::GetRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Run>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_runs].
    fn list_runs(
        &self,
        _req: crate::model::ListRunsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListRunsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_run].
    fn delete_run(
        &self,
        _req: crate::model::DeleteRunRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::create_lineage_event].
    fn create_lineage_event(
        &self,
        _req: crate::model::CreateLineageEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LineageEvent>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_lineage_event].
    fn get_lineage_event(
        &self,
        _req: crate::model::GetLineageEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::LineageEvent>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_lineage_events].
    fn list_lineage_events(
        &self,
        _req: crate::model::ListLineageEventsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListLineageEventsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_lineage_event].
    fn delete_lineage_event(
        &self,
        _req: crate::model::DeleteLineageEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::search_links].
    fn search_links(
        &self,
        _req: crate::model::SearchLinksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::SearchLinksResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::batch_search_link_processes].
    fn batch_search_link_processes(
        &self,
        _req: crate::model::BatchSearchLinkProcessesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::BatchSearchLinkProcessesResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<longrunning::model::Operation>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::Lineage::cancel_operation].
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

