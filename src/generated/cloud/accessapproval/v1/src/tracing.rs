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

/// Implements a [AccessApproval](super::stub::AccessApproval) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct AccessApproval<T>
where T: super::stub::AccessApproval + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct AccessApproval<T>
where T: super::stub::AccessApproval + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> AccessApproval<T>
where T: super::stub::AccessApproval + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> AccessApproval<T>
where T: super::stub::AccessApproval + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::AccessApproval for AccessApproval<T>
where T: super::stub::AccessApproval + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn list_approval_requests(
        &self,
        req: crate::model::ListApprovalRequestsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListApprovalRequestsResponse>> {
        self.inner.list_approval_requests(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_approval_request(
        &self,
        req: crate::model::GetApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.get_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn approve_approval_request(
        &self,
        req: crate::model::ApproveApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.approve_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn dismiss_approval_request(
        &self,
        req: crate::model::DismissApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.dismiss_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn invalidate_approval_request(
        &self,
        req: crate::model::InvalidateApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.invalidate_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_access_approval_settings(
        &self,
        req: crate::model::GetAccessApprovalSettingsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AccessApprovalSettings>> {
        self.inner.get_access_approval_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_access_approval_settings(
        &self,
        req: crate::model::UpdateAccessApprovalSettingsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AccessApprovalSettings>> {
        self.inner.update_access_approval_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_access_approval_settings(
        &self,
        req: crate::model::DeleteAccessApprovalSettingsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_access_approval_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_access_approval_service_account(
        &self,
        req: crate::model::GetAccessApprovalServiceAccountMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AccessApprovalServiceAccount>> {
        self.inner.get_access_approval_service_account(req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::AccessApproval for AccessApproval<T>
where T: super::stub::AccessApproval + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn list_approval_requests(
        &self,
        req: crate::model::ListApprovalRequestsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListApprovalRequestsResponse>> {
        self.inner.list_approval_requests(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_approval_request(
        &self,
        req: crate::model::GetApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.get_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn approve_approval_request(
        &self,
        req: crate::model::ApproveApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.approve_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn dismiss_approval_request(
        &self,
        req: crate::model::DismissApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.dismiss_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn invalidate_approval_request(
        &self,
        req: crate::model::InvalidateApprovalRequestMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ApprovalRequest>> {
        self.inner.invalidate_approval_request(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_access_approval_settings(
        &self,
        req: crate::model::GetAccessApprovalSettingsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AccessApprovalSettings>> {
        self.inner.get_access_approval_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_access_approval_settings(
        &self,
        req: crate::model::UpdateAccessApprovalSettingsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AccessApprovalSettings>> {
        self.inner.update_access_approval_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_access_approval_settings(
        &self,
        req: crate::model::DeleteAccessApprovalSettingsMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_access_approval_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_access_approval_service_account(
        &self,
        req: crate::model::GetAccessApprovalServiceAccountMessage,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AccessApprovalServiceAccount>> {
        self.inner.get_access_approval_service_account(req, options).await
    }

}

