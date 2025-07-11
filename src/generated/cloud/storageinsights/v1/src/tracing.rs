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

/// Implements a [StorageInsights](super::stub::StorageInsights) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct StorageInsights<T>
where T: super::stub::StorageInsights + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct StorageInsights<T>
where T: super::stub::StorageInsights + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> StorageInsights<T>
where T: super::stub::StorageInsights + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> StorageInsights<T>
where T: super::stub::StorageInsights + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::StorageInsights for StorageInsights<T>
where T: super::stub::StorageInsights + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListReportConfigsResponse>> {
        self.inner.list_report_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportConfig>> {
        self.inner.get_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportConfig>> {
        self.inner.create_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_report_config(
        &self,
        req: crate::model::UpdateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportConfig>> {
        self.inner.update_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_report_details(
        &self,
        req: crate::model::ListReportDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListReportDetailsResponse>> {
        self.inner.list_report_details(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_report_detail(
        &self,
        req: crate::model::GetReportDetailRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportDetail>> {
        self.inner.get_report_detail(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_dataset_configs(
        &self,
        req: crate::model::ListDatasetConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatasetConfigsResponse>> {
        self.inner.list_dataset_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dataset_config(
        &self,
        req: crate::model::GetDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DatasetConfig>> {
        self.inner.get_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_dataset_config(
        &self,
        req: crate::model::CreateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_dataset_config(
        &self,
        req: crate::model::UpdateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dataset_config(
        &self,
        req: crate::model::DeleteDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn link_dataset(
        &self,
        req: crate::model::LinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.link_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn unlink_dataset(
        &self,
        req: crate::model::UnlinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.unlink_dataset(req, options).await
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
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
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
impl<T> super::stub::StorageInsights for StorageInsights<T>
where T: super::stub::StorageInsights + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListReportConfigsResponse>> {
        self.inner.list_report_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportConfig>> {
        self.inner.get_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportConfig>> {
        self.inner.create_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_report_config(
        &self,
        req: crate::model::UpdateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportConfig>> {
        self.inner.update_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_report_details(
        &self,
        req: crate::model::ListReportDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListReportDetailsResponse>> {
        self.inner.list_report_details(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_report_detail(
        &self,
        req: crate::model::GetReportDetailRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ReportDetail>> {
        self.inner.get_report_detail(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_dataset_configs(
        &self,
        req: crate::model::ListDatasetConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatasetConfigsResponse>> {
        self.inner.list_dataset_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dataset_config(
        &self,
        req: crate::model::GetDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DatasetConfig>> {
        self.inner.get_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_dataset_config(
        &self,
        req: crate::model::CreateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_dataset_config(
        &self,
        req: crate::model::UpdateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dataset_config(
        &self,
        req: crate::model::DeleteDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_dataset_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn link_dataset(
        &self,
        req: crate::model::LinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.link_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn unlink_dataset(
        &self,
        req: crate::model::UnlinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.unlink_dataset(req, options).await
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
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
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

