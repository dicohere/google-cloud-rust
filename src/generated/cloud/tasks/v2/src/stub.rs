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

/// Defines the trait used to implement [super::client::CloudTasks].
///
/// Application developers may need to implement this trait to mock
/// `client::CloudTasks`.  In other use-cases, application developers only
/// use `client::CloudTasks` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait CloudTasks: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::CloudTasks::list_queues].
    fn list_queues(
        &self,
        _req: crate::model::ListQueuesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListQueuesResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_queue].
    fn get_queue(
        &self,
        _req: crate::model::GetQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::create_queue].
    fn create_queue(
        &self,
        _req: crate::model::CreateQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::update_queue].
    fn update_queue(
        &self,
        _req: crate::model::UpdateQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::delete_queue].
    fn delete_queue(
        &self,
        _req: crate::model::DeleteQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::purge_queue].
    fn purge_queue(
        &self,
        _req: crate::model::PurgeQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::pause_queue].
    fn pause_queue(
        &self,
        _req: crate::model::PauseQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::resume_queue].
    fn resume_queue(
        &self,
        _req: crate::model::ResumeQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::list_tasks].
    fn list_tasks(
        &self,
        _req: crate::model::ListTasksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListTasksResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_task].
    fn get_task(
        &self,
        _req: crate::model::GetTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Task>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::create_task].
    fn create_task(
        &self,
        _req: crate::model::CreateTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Task>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::delete_task].
    fn delete_task(
        &self,
        _req: crate::model::DeleteTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::run_task].
    fn run_task(
        &self,
        _req: crate::model::RunTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Task>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait CloudTasks: std::fmt::Debug {

    /// Implements [super::client::CloudTasks::list_queues].
    fn list_queues(
        &self,
        _req: crate::model::ListQueuesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListQueuesResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_queue].
    fn get_queue(
        &self,
        _req: crate::model::GetQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::create_queue].
    fn create_queue(
        &self,
        _req: crate::model::CreateQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::update_queue].
    fn update_queue(
        &self,
        _req: crate::model::UpdateQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::delete_queue].
    fn delete_queue(
        &self,
        _req: crate::model::DeleteQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::purge_queue].
    fn purge_queue(
        &self,
        _req: crate::model::PurgeQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::pause_queue].
    fn pause_queue(
        &self,
        _req: crate::model::PauseQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::resume_queue].
    fn resume_queue(
        &self,
        _req: crate::model::ResumeQueueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Queue>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::list_tasks].
    fn list_tasks(
        &self,
        _req: crate::model::ListTasksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::ListTasksResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_task].
    fn get_task(
        &self,
        _req: crate::model::GetTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Task>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::create_task].
    fn create_task(
        &self,
        _req: crate::model::CreateTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Task>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::delete_task].
    fn delete_task(
        &self,
        _req: crate::model::DeleteTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::run_task].
    fn run_task(
        &self,
        _req: crate::model::RunTaskRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::Task>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::CloudTasks::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<location::model::Location>>> {
        gaxi::unimplemented::unimplemented_stub()
    }
}

