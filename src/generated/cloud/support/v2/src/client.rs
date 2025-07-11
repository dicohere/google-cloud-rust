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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

/// Implements a client for the Google Cloud Support API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_support_v2::client::CaseAttachmentService;
/// let client = CaseAttachmentService::builder().build().await?;
/// // use `client` to make requests to the Google Cloud Support API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service to manage file attachments for Google Cloud support cases.
///
/// # Configuration
///
/// To configure `CaseAttachmentService` use the `with_*` methods in the type returned
/// by [builder()][CaseAttachmentService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudsupport.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::case_attachment_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::case_attachment_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CaseAttachmentService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CaseAttachmentService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CaseAttachmentService {
    inner: std::sync::Arc<dyn super::stub::dynamic::CaseAttachmentService>,
}

impl CaseAttachmentService {
    /// Returns a builder for [CaseAttachmentService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_support_v2::client::CaseAttachmentService;
    /// let client = CaseAttachmentService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::case_attachment_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::case_attachment_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CaseAttachmentService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CaseAttachmentService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CaseAttachmentService> {
        super::transport::CaseAttachmentService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CaseAttachmentService> {
        Self::build_transport(conf).await.map(super::tracing::CaseAttachmentService::new)
    }

    /// List all the attachments associated with a support case.
    pub fn list_attachments(&self) -> super::builder::case_attachment_service::ListAttachments
    {
        super::builder::case_attachment_service::ListAttachments::new(self.inner.clone())
    }
}

/// Implements a client for the Google Cloud Support API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_support_v2::client::CaseService;
/// let client = CaseService::builder().build().await?;
/// // use `client` to make requests to the Google Cloud Support API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service to manage Google Cloud support cases.
///
/// # Configuration
///
/// To configure `CaseService` use the `with_*` methods in the type returned
/// by [builder()][CaseService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudsupport.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::case_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::case_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CaseService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CaseService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CaseService {
    inner: std::sync::Arc<dyn super::stub::dynamic::CaseService>,
}

impl CaseService {
    /// Returns a builder for [CaseService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_support_v2::client::CaseService;
    /// let client = CaseService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::case_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::case_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CaseService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CaseService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CaseService> {
        super::transport::CaseService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CaseService> {
        Self::build_transport(conf).await.map(super::tracing::CaseService::new)
    }

    /// Retrieve a case.
    pub fn get_case(&self) -> super::builder::case_service::GetCase
    {
        super::builder::case_service::GetCase::new(self.inner.clone())
    }

    /// Retrieve all cases under a parent, but not its children.
    ///
    /// For example, listing cases under an organization only returns the cases
    /// that are directly parented by that organization. To retrieve cases
    /// under an organization and its projects, use `cases.search`.
    pub fn list_cases(&self) -> super::builder::case_service::ListCases
    {
        super::builder::case_service::ListCases::new(self.inner.clone())
    }

    /// Search for cases using a query.
    pub fn search_cases(&self) -> super::builder::case_service::SearchCases
    {
        super::builder::case_service::SearchCases::new(self.inner.clone())
    }

    /// Create a new case and associate it with a parent.
    ///
    /// It must have the following fields set: `display_name`, `description`,
    /// `classification`, and `priority`. If you're just testing the API and don't
    /// want to route your case to an agent, set `testCase=true`.
    pub fn create_case(&self) -> super::builder::case_service::CreateCase
    {
        super::builder::case_service::CreateCase::new(self.inner.clone())
    }

    /// Update a case. Only some fields can be updated.
    pub fn update_case(&self) -> super::builder::case_service::UpdateCase
    {
        super::builder::case_service::UpdateCase::new(self.inner.clone())
    }

    /// Escalate a case, starting the Google Cloud Support escalation management
    /// process.
    ///
    /// This operation is only available for some support services. Go to
    /// <https://cloud.google.com/support> and look for 'Technical support
    /// escalations' in the feature list to find out which ones let you
    /// do that.
    pub fn escalate_case(&self) -> super::builder::case_service::EscalateCase
    {
        super::builder::case_service::EscalateCase::new(self.inner.clone())
    }

    /// Close a case.
    pub fn close_case(&self) -> super::builder::case_service::CloseCase
    {
        super::builder::case_service::CloseCase::new(self.inner.clone())
    }

    /// Retrieve valid classifications to use when creating a support case.
    ///
    /// Classifications are hierarchical. Each classification is a string
    /// containing all levels of the hierarchy separated by `" > "`. For example,
    /// `"Technical Issue > Compute > Compute Engine"`.
    ///
    /// Classification IDs returned by this endpoint are valid for at least six
    /// months. When a classification is deactivated, this endpoint immediately
    /// stops returning it. After six months, `case.create` requests using the
    /// classification will fail.
    pub fn search_case_classifications(&self) -> super::builder::case_service::SearchCaseClassifications
    {
        super::builder::case_service::SearchCaseClassifications::new(self.inner.clone())
    }
}

/// Implements a client for the Google Cloud Support API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_support_v2::client::CommentService;
/// let client = CommentService::builder().build().await?;
/// // use `client` to make requests to the Google Cloud Support API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service to manage comments on cases.
///
/// # Configuration
///
/// To configure `CommentService` use the `with_*` methods in the type returned
/// by [builder()][CommentService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudsupport.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::comment_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::comment_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CommentService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CommentService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CommentService {
    inner: std::sync::Arc<dyn super::stub::dynamic::CommentService>,
}

impl CommentService {
    /// Returns a builder for [CommentService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_support_v2::client::CommentService;
    /// let client = CommentService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::comment_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::comment_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CommentService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CommentService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CommentService> {
        super::transport::CommentService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CommentService> {
        Self::build_transport(conf).await.map(super::tracing::CommentService::new)
    }

    /// List all the comments associated with a case.
    pub fn list_comments(&self) -> super::builder::comment_service::ListComments
    {
        super::builder::comment_service::ListComments::new(self.inner.clone())
    }

    /// Add a new comment to a case.
    ///
    /// The comment must have the following fields set: `body`.
    pub fn create_comment(&self) -> super::builder::comment_service::CreateComment
    {
        super::builder::comment_service::CreateComment::new(self.inner.clone())
    }
}
