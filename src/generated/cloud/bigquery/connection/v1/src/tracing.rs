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

/// Implements a [ConnectionService](super::stub::ConnectionService) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct ConnectionService<T>
where T: super::stub::ConnectionService + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct ConnectionService<T>
where T: super::stub::ConnectionService + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> ConnectionService<T>
where T: super::stub::ConnectionService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> ConnectionService<T>
where T: super::stub::ConnectionService + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::ConnectionService for ConnectionService<T>
where T: super::stub::ConnectionService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Connection>> {
        self.inner.create_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Connection>> {
        self.inner.get_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListConnectionsResponse>> {
        self.inner.list_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Connection>> {
        self.inner.update_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_connection(req, options).await
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

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::ConnectionService for ConnectionService<T>
where T: super::stub::ConnectionService + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Connection>> {
        self.inner.create_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Connection>> {
        self.inner.get_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListConnectionsResponse>> {
        self.inner.list_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Connection>> {
        self.inner.update_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_connection(req, options).await
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

}

