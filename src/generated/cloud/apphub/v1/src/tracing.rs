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

/// Implements a [AppHub](super::stub::AppHub) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct AppHub<T>
where T: super::stub::AppHub + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct AppHub<T>
where T: super::stub::AppHub + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> AppHub<T>
where T: super::stub::AppHub + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> AppHub<T>
where T: super::stub::AppHub + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::AppHub for AppHub<T>
where T: super::stub::AppHub + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn lookup_service_project_attachment(
        &self,
        req: crate::model::LookupServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LookupServiceProjectAttachmentResponse>> {
        self.inner.lookup_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_service_project_attachments(
        &self,
        req: crate::model::ListServiceProjectAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListServiceProjectAttachmentsResponse>> {
        self.inner.list_service_project_attachments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service_project_attachment(
        &self,
        req: crate::model::CreateServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service_project_attachment(
        &self,
        req: crate::model::GetServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceProjectAttachment>> {
        self.inner.get_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service_project_attachment(
        &self,
        req: crate::model::DeleteServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn detach_service_project_attachment(
        &self,
        req: crate::model::DetachServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DetachServiceProjectAttachmentResponse>> {
        self.inner.detach_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_discovered_services(
        &self,
        req: crate::model::ListDiscoveredServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDiscoveredServicesResponse>> {
        self.inner.list_discovered_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_discovered_service(
        &self,
        req: crate::model::GetDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DiscoveredService>> {
        self.inner.get_discovered_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_discovered_service(
        &self,
        req: crate::model::LookupDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LookupDiscoveredServiceResponse>> {
        self.inner.lookup_discovered_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListServicesResponse>> {
        self.inner.list_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Service>> {
        self.inner.get_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_discovered_workloads(
        &self,
        req: crate::model::ListDiscoveredWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDiscoveredWorkloadsResponse>> {
        self.inner.list_discovered_workloads(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_discovered_workload(
        &self,
        req: crate::model::GetDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DiscoveredWorkload>> {
        self.inner.get_discovered_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_discovered_workload(
        &self,
        req: crate::model::LookupDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LookupDiscoveredWorkloadResponse>> {
        self.inner.lookup_discovered_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListWorkloadsResponse>> {
        self.inner.list_workloads(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_workload(
        &self,
        req: crate::model::CreateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workload(
        &self,
        req: crate::model::GetWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Workload>> {
        self.inner.get_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workload(
        &self,
        req: crate::model::UpdateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workload(
        &self,
        req: crate::model::DeleteWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_applications(
        &self,
        req: crate::model::ListApplicationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListApplicationsResponse>> {
        self.inner.list_applications(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_application(
        &self,
        req: crate::model::CreateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_application(
        &self,
        req: crate::model::GetApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Application>> {
        self.inner.get_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_application(
        &self,
        req: crate::model::UpdateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_application(
        &self,
        req: crate::model::DeleteApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_application(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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
impl<T> super::stub::AppHub for AppHub<T>
where T: super::stub::AppHub + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn lookup_service_project_attachment(
        &self,
        req: crate::model::LookupServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LookupServiceProjectAttachmentResponse>> {
        self.inner.lookup_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_service_project_attachments(
        &self,
        req: crate::model::ListServiceProjectAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListServiceProjectAttachmentsResponse>> {
        self.inner.list_service_project_attachments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service_project_attachment(
        &self,
        req: crate::model::CreateServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service_project_attachment(
        &self,
        req: crate::model::GetServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceProjectAttachment>> {
        self.inner.get_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service_project_attachment(
        &self,
        req: crate::model::DeleteServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn detach_service_project_attachment(
        &self,
        req: crate::model::DetachServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DetachServiceProjectAttachmentResponse>> {
        self.inner.detach_service_project_attachment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_discovered_services(
        &self,
        req: crate::model::ListDiscoveredServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDiscoveredServicesResponse>> {
        self.inner.list_discovered_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_discovered_service(
        &self,
        req: crate::model::GetDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DiscoveredService>> {
        self.inner.get_discovered_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_discovered_service(
        &self,
        req: crate::model::LookupDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LookupDiscoveredServiceResponse>> {
        self.inner.lookup_discovered_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListServicesResponse>> {
        self.inner.list_services(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Service>> {
        self.inner.get_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_discovered_workloads(
        &self,
        req: crate::model::ListDiscoveredWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDiscoveredWorkloadsResponse>> {
        self.inner.list_discovered_workloads(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_discovered_workload(
        &self,
        req: crate::model::GetDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DiscoveredWorkload>> {
        self.inner.get_discovered_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_discovered_workload(
        &self,
        req: crate::model::LookupDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LookupDiscoveredWorkloadResponse>> {
        self.inner.lookup_discovered_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListWorkloadsResponse>> {
        self.inner.list_workloads(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_workload(
        &self,
        req: crate::model::CreateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_workload(
        &self,
        req: crate::model::GetWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Workload>> {
        self.inner.get_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_workload(
        &self,
        req: crate::model::UpdateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_workload(
        &self,
        req: crate::model::DeleteWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_workload(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_applications(
        &self,
        req: crate::model::ListApplicationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListApplicationsResponse>> {
        self.inner.list_applications(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_application(
        &self,
        req: crate::model::CreateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_application(
        &self,
        req: crate::model::GetApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Application>> {
        self.inner.get_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_application(
        &self,
        req: crate::model::UpdateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_application(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_application(
        &self,
        req: crate::model::DeleteApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_application(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        self.inner.test_iam_permissions(req, options).await
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

