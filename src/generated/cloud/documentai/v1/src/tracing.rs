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

/// Implements a [DocumentProcessorService](super::stub::DocumentProcessorService) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct DocumentProcessorService<T>
where T: super::stub::DocumentProcessorService + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct DocumentProcessorService<T>
where T: super::stub::DocumentProcessorService + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> DocumentProcessorService<T>
where T: super::stub::DocumentProcessorService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> DocumentProcessorService<T>
where T: super::stub::DocumentProcessorService + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::DocumentProcessorService for DocumentProcessorService<T>
where T: super::stub::DocumentProcessorService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn process_document(
        &self,
        req: crate::model::ProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessResponse>> {
        self.inner.process_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_process_documents(
        &self,
        req: crate::model::BatchProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_process_documents(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_processor_types(
        &self,
        req: crate::model::FetchProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::FetchProcessorTypesResponse>> {
        self.inner.fetch_processor_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_processor_types(
        &self,
        req: crate::model::ListProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessorTypesResponse>> {
        self.inner.list_processor_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_processor_type(
        &self,
        req: crate::model::GetProcessorTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessorType>> {
        self.inner.get_processor_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_processors(
        &self,
        req: crate::model::ListProcessorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessorsResponse>> {
        self.inner.list_processors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_processor(
        &self,
        req: crate::model::GetProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Processor>> {
        self.inner.get_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn train_processor_version(
        &self,
        req: crate::model::TrainProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.train_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_processor_version(
        &self,
        req: crate::model::GetProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessorVersion>> {
        self.inner.get_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_processor_versions(
        &self,
        req: crate::model::ListProcessorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessorVersionsResponse>> {
        self.inner.list_processor_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_processor_version(
        &self,
        req: crate::model::DeleteProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn deploy_processor_version(
        &self,
        req: crate::model::DeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.deploy_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undeploy_processor_version(
        &self,
        req: crate::model::UndeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.undeploy_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_processor(
        &self,
        req: crate::model::CreateProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Processor>> {
        self.inner.create_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_processor(
        &self,
        req: crate::model::DeleteProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_processor(
        &self,
        req: crate::model::EnableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.enable_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_processor(
        &self,
        req: crate::model::DisableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.disable_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_default_processor_version(
        &self,
        req: crate::model::SetDefaultProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.set_default_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn review_document(
        &self,
        req: crate::model::ReviewDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.review_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn evaluate_processor_version(
        &self,
        req: crate::model::EvaluateProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.evaluate_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_evaluation(
        &self,
        req: crate::model::GetEvaluationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Evaluation>> {
        self.inner.get_evaluation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_evaluations(
        &self,
        req: crate::model::ListEvaluationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListEvaluationsResponse>> {
        self.inner.list_evaluations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
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
impl<T> super::stub::DocumentProcessorService for DocumentProcessorService<T>
where T: super::stub::DocumentProcessorService + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn process_document(
        &self,
        req: crate::model::ProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessResponse>> {
        self.inner.process_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_process_documents(
        &self,
        req: crate::model::BatchProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_process_documents(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_processor_types(
        &self,
        req: crate::model::FetchProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::FetchProcessorTypesResponse>> {
        self.inner.fetch_processor_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_processor_types(
        &self,
        req: crate::model::ListProcessorTypesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessorTypesResponse>> {
        self.inner.list_processor_types(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_processor_type(
        &self,
        req: crate::model::GetProcessorTypeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessorType>> {
        self.inner.get_processor_type(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_processors(
        &self,
        req: crate::model::ListProcessorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessorsResponse>> {
        self.inner.list_processors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_processor(
        &self,
        req: crate::model::GetProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Processor>> {
        self.inner.get_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn train_processor_version(
        &self,
        req: crate::model::TrainProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.train_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_processor_version(
        &self,
        req: crate::model::GetProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessorVersion>> {
        self.inner.get_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_processor_versions(
        &self,
        req: crate::model::ListProcessorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessorVersionsResponse>> {
        self.inner.list_processor_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_processor_version(
        &self,
        req: crate::model::DeleteProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn deploy_processor_version(
        &self,
        req: crate::model::DeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.deploy_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undeploy_processor_version(
        &self,
        req: crate::model::UndeployProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.undeploy_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_processor(
        &self,
        req: crate::model::CreateProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Processor>> {
        self.inner.create_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_processor(
        &self,
        req: crate::model::DeleteProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_processor(
        &self,
        req: crate::model::EnableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.enable_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_processor(
        &self,
        req: crate::model::DisableProcessorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.disable_processor(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_default_processor_version(
        &self,
        req: crate::model::SetDefaultProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.set_default_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn review_document(
        &self,
        req: crate::model::ReviewDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.review_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn evaluate_processor_version(
        &self,
        req: crate::model::EvaluateProcessorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.evaluate_processor_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_evaluation(
        &self,
        req: crate::model::GetEvaluationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Evaluation>> {
        self.inner.get_evaluation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_evaluations(
        &self,
        req: crate::model::ListEvaluationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListEvaluationsResponse>> {
        self.inner.list_evaluations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
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

