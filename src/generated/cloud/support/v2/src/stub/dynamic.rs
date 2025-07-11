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

/// A dyn-compatible, crate-private version of [super::CaseAttachmentService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait CaseAttachmentService: std::fmt::Debug + Send + Sync {
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAttachmentsResponse>>;

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait CaseAttachmentService: std::fmt::Debug {
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAttachmentsResponse>>;

}

/// All implementations of [super::CaseAttachmentService] also implement [CaseAttachmentService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::CaseAttachmentService> CaseAttachmentService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAttachmentsResponse>> {
        T::list_attachments(self, req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::CaseAttachmentService> CaseAttachmentService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_attachments(
        &self,
        req: crate::model::ListAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListAttachmentsResponse>> {
        T::list_attachments(self, req, options).await
    }

}

/// A dyn-compatible, crate-private version of [super::CaseService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait CaseService: std::fmt::Debug + Send + Sync {
    async fn get_case(
        &self,
        req: crate::model::GetCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn list_cases(
        &self,
        req: crate::model::ListCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCasesResponse>>;

    async fn search_cases(
        &self,
        req: crate::model::SearchCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCasesResponse>>;

    async fn create_case(
        &self,
        req: crate::model::CreateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn update_case(
        &self,
        req: crate::model::UpdateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn escalate_case(
        &self,
        req: crate::model::EscalateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn close_case(
        &self,
        req: crate::model::CloseCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn search_case_classifications(
        &self,
        req: crate::model::SearchCaseClassificationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCaseClassificationsResponse>>;

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait CaseService: std::fmt::Debug {
    async fn get_case(
        &self,
        req: crate::model::GetCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn list_cases(
        &self,
        req: crate::model::ListCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCasesResponse>>;

    async fn search_cases(
        &self,
        req: crate::model::SearchCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCasesResponse>>;

    async fn create_case(
        &self,
        req: crate::model::CreateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn update_case(
        &self,
        req: crate::model::UpdateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn escalate_case(
        &self,
        req: crate::model::EscalateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn close_case(
        &self,
        req: crate::model::CloseCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>>;

    async fn search_case_classifications(
        &self,
        req: crate::model::SearchCaseClassificationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCaseClassificationsResponse>>;

}

/// All implementations of [super::CaseService] also implement [CaseService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::CaseService> CaseService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_case(
        &self,
        req: crate::model::GetCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::get_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_cases(
        &self,
        req: crate::model::ListCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCasesResponse>> {
        T::list_cases(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_cases(
        &self,
        req: crate::model::SearchCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCasesResponse>> {
        T::search_cases(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_case(
        &self,
        req: crate::model::CreateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::create_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_case(
        &self,
        req: crate::model::UpdateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::update_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn escalate_case(
        &self,
        req: crate::model::EscalateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::escalate_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn close_case(
        &self,
        req: crate::model::CloseCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::close_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_case_classifications(
        &self,
        req: crate::model::SearchCaseClassificationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCaseClassificationsResponse>> {
        T::search_case_classifications(self, req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::CaseService> CaseService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_case(
        &self,
        req: crate::model::GetCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::get_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_cases(
        &self,
        req: crate::model::ListCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCasesResponse>> {
        T::list_cases(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_cases(
        &self,
        req: crate::model::SearchCasesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCasesResponse>> {
        T::search_cases(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_case(
        &self,
        req: crate::model::CreateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::create_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_case(
        &self,
        req: crate::model::UpdateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::update_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn escalate_case(
        &self,
        req: crate::model::EscalateCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::escalate_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn close_case(
        &self,
        req: crate::model::CloseCaseRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Case>> {
        T::close_case(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn search_case_classifications(
        &self,
        req: crate::model::SearchCaseClassificationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::SearchCaseClassificationsResponse>> {
        T::search_case_classifications(self, req, options).await
    }

}

/// A dyn-compatible, crate-private version of [super::CommentService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait CommentService: std::fmt::Debug + Send + Sync {
    async fn list_comments(
        &self,
        req: crate::model::ListCommentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCommentsResponse>>;

    async fn create_comment(
        &self,
        req: crate::model::CreateCommentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Comment>>;

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait CommentService: std::fmt::Debug {
    async fn list_comments(
        &self,
        req: crate::model::ListCommentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCommentsResponse>>;

    async fn create_comment(
        &self,
        req: crate::model::CreateCommentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Comment>>;

}

/// All implementations of [super::CommentService] also implement [CommentService].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::CommentService> CommentService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_comments(
        &self,
        req: crate::model::ListCommentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCommentsResponse>> {
        T::list_comments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_comment(
        &self,
        req: crate::model::CreateCommentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Comment>> {
        T::create_comment(self, req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::CommentService> CommentService for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_comments(
        &self,
        req: crate::model::ListCommentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListCommentsResponse>> {
        T::list_comments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_comment(
        &self,
        req: crate::model::CreateCommentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Comment>> {
        T::create_comment(self, req, options).await
    }

}
