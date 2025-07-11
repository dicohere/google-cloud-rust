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
use crate::Result;

/// Implements a [CloudBuild](super::stub::CloudBuild) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct CloudBuild<T>
where T: super::stub::CloudBuild + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct CloudBuild<T>
where T: super::stub::CloudBuild + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> CloudBuild<T>
where T: super::stub::CloudBuild + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> CloudBuild<T>
where T: super::stub::CloudBuild + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::CloudBuild for CloudBuild<T>
where T: super::stub::CloudBuild + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn create_build(
        &self,
        req: crate::model::CreateBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_build(
        &self,
        req: crate::model::GetBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Build>> {
        self.inner.get_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_builds(
        &self,
        req: crate::model::ListBuildsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBuildsResponse>> {
        self.inner.list_builds(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_build(
        &self,
        req: crate::model::CancelBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Build>> {
        self.inner.cancel_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retry_build(
        &self,
        req: crate::model::RetryBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.retry_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn approve_build(
        &self,
        req: crate::model::ApproveBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.approve_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_build_trigger(
        &self,
        req: crate::model::CreateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BuildTrigger>> {
        self.inner.create_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_build_trigger(
        &self,
        req: crate::model::GetBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BuildTrigger>> {
        self.inner.get_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_build_triggers(
        &self,
        req: crate::model::ListBuildTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBuildTriggersResponse>> {
        self.inner.list_build_triggers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_build_trigger(
        &self,
        req: crate::model::DeleteBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_build_trigger(
        &self,
        req: crate::model::UpdateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BuildTrigger>> {
        self.inner.update_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn run_build_trigger(
        &self,
        req: crate::model::RunBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.run_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn receive_trigger_webhook(
        &self,
        req: crate::model::ReceiveTriggerWebhookRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReceiveTriggerWebhookResponse>> {
        self.inner.receive_trigger_webhook(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_worker_pool(
        &self,
        req: crate::model::CreateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_worker_pool(
        &self,
        req: crate::model::GetWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::WorkerPool>> {
        self.inner.get_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_worker_pool(
        &self,
        req: crate::model::DeleteWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_worker_pool(
        &self,
        req: crate::model::UpdateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_worker_pools(
        &self,
        req: crate::model::ListWorkerPoolsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListWorkerPoolsResponse>> {
        self.inner.list_worker_pools(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }


    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::CloudBuild for CloudBuild<T>
where T: super::stub::CloudBuild + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn create_build(
        &self,
        req: crate::model::CreateBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_build(
        &self,
        req: crate::model::GetBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Build>> {
        self.inner.get_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_builds(
        &self,
        req: crate::model::ListBuildsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBuildsResponse>> {
        self.inner.list_builds(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_build(
        &self,
        req: crate::model::CancelBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Build>> {
        self.inner.cancel_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn retry_build(
        &self,
        req: crate::model::RetryBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.retry_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn approve_build(
        &self,
        req: crate::model::ApproveBuildRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.approve_build(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_build_trigger(
        &self,
        req: crate::model::CreateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BuildTrigger>> {
        self.inner.create_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_build_trigger(
        &self,
        req: crate::model::GetBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BuildTrigger>> {
        self.inner.get_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_build_triggers(
        &self,
        req: crate::model::ListBuildTriggersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListBuildTriggersResponse>> {
        self.inner.list_build_triggers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_build_trigger(
        &self,
        req: crate::model::DeleteBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_build_trigger(
        &self,
        req: crate::model::UpdateBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BuildTrigger>> {
        self.inner.update_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn run_build_trigger(
        &self,
        req: crate::model::RunBuildTriggerRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.run_build_trigger(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn receive_trigger_webhook(
        &self,
        req: crate::model::ReceiveTriggerWebhookRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReceiveTriggerWebhookResponse>> {
        self.inner.receive_trigger_webhook(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_worker_pool(
        &self,
        req: crate::model::CreateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_worker_pool(
        &self,
        req: crate::model::GetWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::WorkerPool>> {
        self.inner.get_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_worker_pool(
        &self,
        req: crate::model::DeleteWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_worker_pool(
        &self,
        req: crate::model::UpdateWorkerPoolRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_worker_pool(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_worker_pools(
        &self,
        req: crate::model::ListWorkerPoolsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListWorkerPoolsResponse>> {
        self.inner.list_worker_pools(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }


    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

