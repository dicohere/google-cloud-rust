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

/// Implements a [LookupService](super::stub::LookupService) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct LookupService<T>
where T: super::stub::LookupService + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct LookupService<T>
where T: super::stub::LookupService + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> LookupService<T>
where T: super::stub::LookupService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> LookupService<T>
where T: super::stub::LookupService + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::LookupService for LookupService<T>
where T: super::stub::LookupService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn resolve_service(
        &self,
        req: crate::model::ResolveServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ResolveServiceResponse>> {
        self.inner.resolve_service(req, options).await
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

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::LookupService for LookupService<T>
where T: super::stub::LookupService + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn resolve_service(
        &self,
        req: crate::model::ResolveServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ResolveServiceResponse>> {
        self.inner.resolve_service(req, options).await
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

}

/// Implements a [RegistrationService](super::stub::RegistrationService) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct RegistrationService<T>
where T: super::stub::RegistrationService + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct RegistrationService<T>
where T: super::stub::RegistrationService + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> RegistrationService<T>
where T: super::stub::RegistrationService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> RegistrationService<T>
where T: super::stub::RegistrationService + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::RegistrationService for RegistrationService<T>
where T: super::stub::RegistrationService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn create_namespace(
        &self,
        req: crate::model::CreateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Namespace>> {
        self.inner.create_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_namespaces(
        &self,
        req: crate::model::ListNamespacesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListNamespacesResponse>> {
        self.inner.list_namespaces(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_namespace(
        &self,
        req: crate::model::GetNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Namespace>> {
        self.inner.get_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_namespace(
        &self,
        req: crate::model::UpdateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Namespace>> {
        self.inner.update_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_namespace(
        &self,
        req: crate::model::DeleteNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Service>> {
        self.inner.create_service(req, options).await
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
    ) -> Result<gax::response::Response<crate::model::Service>> {
        self.inner.update_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_endpoint(
        &self,
        req: crate::model::CreateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Endpoint>> {
        self.inner.create_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_endpoints(
        &self,
        req: crate::model::ListEndpointsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListEndpointsResponse>> {
        self.inner.list_endpoints(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_endpoint(
        &self,
        req: crate::model::GetEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Endpoint>> {
        self.inner.get_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_endpoint(
        &self,
        req: crate::model::UpdateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Endpoint>> {
        self.inner.update_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_endpoint(
        &self,
        req: crate::model::DeleteEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_endpoint(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
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

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::RegistrationService for RegistrationService<T>
where T: super::stub::RegistrationService + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn create_namespace(
        &self,
        req: crate::model::CreateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Namespace>> {
        self.inner.create_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_namespaces(
        &self,
        req: crate::model::ListNamespacesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListNamespacesResponse>> {
        self.inner.list_namespaces(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_namespace(
        &self,
        req: crate::model::GetNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Namespace>> {
        self.inner.get_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_namespace(
        &self,
        req: crate::model::UpdateNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Namespace>> {
        self.inner.update_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_namespace(
        &self,
        req: crate::model::DeleteNamespaceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_namespace(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Service>> {
        self.inner.create_service(req, options).await
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
    ) -> Result<gax::response::Response<crate::model::Service>> {
        self.inner.update_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_service(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_endpoint(
        &self,
        req: crate::model::CreateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Endpoint>> {
        self.inner.create_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_endpoints(
        &self,
        req: crate::model::ListEndpointsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListEndpointsResponse>> {
        self.inner.list_endpoints(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_endpoint(
        &self,
        req: crate::model::GetEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Endpoint>> {
        self.inner.get_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_endpoint(
        &self,
        req: crate::model::UpdateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Endpoint>> {
        self.inner.update_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_endpoint(
        &self,
        req: crate::model::DeleteEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_endpoint(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        self.inner.set_iam_policy(req, options).await
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

}

