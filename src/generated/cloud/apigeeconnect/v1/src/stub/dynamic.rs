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

/// A dyn-compatible, crate-private version of [super::ConnectionService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait ConnectionService: std::fmt::Debug + Send + Sync {
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>;

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait ConnectionService: std::fmt::Debug {
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>>;

}

/// All implementations of [super::ConnectionService] also implement [ConnectionService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::ConnectionService> ConnectionService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>> {
        T::list_connections(self, req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::ConnectionService> ConnectionService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListConnectionsResponse>> {
        T::list_connections(self, req, options).await
    }

}
