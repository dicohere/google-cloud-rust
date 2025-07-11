// Copyright 2024 Google LLC
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

/// A dyn-compatible, crate-private version of [super::IAMPolicy].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait IAMPolicy: std::fmt::Debug + Send + Sync {
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>>;

    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>>;

    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>>;

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait IAMPolicy: std::fmt::Debug {
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>>;

    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>>;

    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>>;

}

/// All implementations of [super::IAMPolicy] also implement [IAMPolicy].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::IAMPolicy> IAMPolicy for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>> {
        T::test_iam_permissions(self, req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::IAMPolicy> IAMPolicy for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Policy>> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::TestIamPermissionsResponse>> {
        T::test_iam_permissions(self, req, options).await
    }

}
