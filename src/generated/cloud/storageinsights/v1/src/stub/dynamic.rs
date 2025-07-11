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

/// A dyn-compatible, crate-private version of [super::StorageInsights].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait StorageInsights: std::fmt::Debug + Send + Sync {
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportConfigsResponse>>;

    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>>;

    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>>;

    async fn update_report_config(
        &self,
        req: crate::model::UpdateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>>;

    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_report_details(
        &self,
        req: crate::model::ListReportDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportDetailsResponse>>;

    async fn get_report_detail(
        &self,
        req: crate::model::GetReportDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportDetail>>;

    async fn list_dataset_configs(
        &self,
        req: crate::model::ListDatasetConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDatasetConfigsResponse>>;

    async fn get_dataset_config(
        &self,
        req: crate::model::GetDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DatasetConfig>>;

    async fn create_dataset_config(
        &self,
        req: crate::model::CreateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_dataset_config(
        &self,
        req: crate::model::UpdateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_dataset_config(
        &self,
        req: crate::model::DeleteDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn link_dataset(
        &self,
        req: crate::model::LinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn unlink_dataset(
        &self,
        req: crate::model::UnlinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait StorageInsights: std::fmt::Debug {
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportConfigsResponse>>;

    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>>;

    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>>;

    async fn update_report_config(
        &self,
        req: crate::model::UpdateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>>;

    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn list_report_details(
        &self,
        req: crate::model::ListReportDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportDetailsResponse>>;

    async fn get_report_detail(
        &self,
        req: crate::model::GetReportDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportDetail>>;

    async fn list_dataset_configs(
        &self,
        req: crate::model::ListDatasetConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDatasetConfigsResponse>>;

    async fn get_dataset_config(
        &self,
        req: crate::model::GetDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DatasetConfig>>;

    async fn create_dataset_config(
        &self,
        req: crate::model::CreateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn update_dataset_config(
        &self,
        req: crate::model::UpdateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_dataset_config(
        &self,
        req: crate::model::DeleteDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn link_dataset(
        &self,
        req: crate::model::LinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn unlink_dataset(
        &self,
        req: crate::model::UnlinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::StorageInsights] also implement [StorageInsights].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::StorageInsights> StorageInsights for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportConfigsResponse>> {
        T::list_report_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>> {
        T::get_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>> {
        T::create_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_report_config(
        &self,
        req: crate::model::UpdateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>> {
        T::update_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_report_details(
        &self,
        req: crate::model::ListReportDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportDetailsResponse>> {
        T::list_report_details(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_report_detail(
        &self,
        req: crate::model::GetReportDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportDetail>> {
        T::get_report_detail(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_dataset_configs(
        &self,
        req: crate::model::ListDatasetConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDatasetConfigsResponse>> {
        T::list_dataset_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_dataset_config(
        &self,
        req: crate::model::GetDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DatasetConfig>> {
        T::get_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_dataset_config(
        &self,
        req: crate::model::CreateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_dataset_config(
        &self,
        req: crate::model::UpdateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_dataset_config(
        &self,
        req: crate::model::DeleteDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn link_dataset(
        &self,
        req: crate::model::LinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::link_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn unlink_dataset(
        &self,
        req: crate::model::UnlinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::unlink_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::StorageInsights> StorageInsights for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportConfigsResponse>> {
        T::list_report_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>> {
        T::get_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>> {
        T::create_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_report_config(
        &self,
        req: crate::model::UpdateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportConfig>> {
        T::update_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_report_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_report_details(
        &self,
        req: crate::model::ListReportDetailsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListReportDetailsResponse>> {
        T::list_report_details(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_report_detail(
        &self,
        req: crate::model::GetReportDetailRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ReportDetail>> {
        T::get_report_detail(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_dataset_configs(
        &self,
        req: crate::model::ListDatasetConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDatasetConfigsResponse>> {
        T::list_dataset_configs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_dataset_config(
        &self,
        req: crate::model::GetDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::DatasetConfig>> {
        T::get_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_dataset_config(
        &self,
        req: crate::model::CreateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::create_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_dataset_config(
        &self,
        req: crate::model::UpdateDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::update_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_dataset_config(
        &self,
        req: crate::model::DeleteDatasetConfigRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::delete_dataset_config(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn link_dataset(
        &self,
        req: crate::model::LinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::link_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn unlink_dataset(
        &self,
        req: crate::model::UnlinkDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::unlink_dataset(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::ListLocationsResponse>> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<location::model::Location>> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<longrunning::model::Operation>> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
