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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

/// Implements a client for the IAM Meta API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_iam_v1::client::IAMPolicy;
/// let client = IAMPolicy::builder().build().await?;
/// // use `client` to make requests to the IAM Meta API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// API Overview
///
/// Manages Identity and Access Management (IAM) policies.
///
/// Any implementation of an API that offers access control features
/// implements the google.iam.v1.IAMPolicy interface.
///
/// ## Data model
///
/// Access control is applied when a principal (user or service account), takes
/// some action on a resource exposed by a service. Resources, identified by
/// URI-like names, are the unit of access control specification. Service
/// implementations can choose the granularity of access control and the
/// supported permissions for their resources.
/// For example one database service may allow access control to be
/// specified only at the Table level, whereas another might allow access control
/// to also be specified at the Column level.
///
/// ## Policy Structure
///
/// See google.iam.v1.Policy
///
/// This is intentionally not a CRUD style API because access control policies
/// are created and deleted implicitly with the resources to which they are
/// attached.
///
/// # Configuration
///
/// To configure `IAMPolicy` use the `with_*` methods in the type returned
/// by [builder()][IAMPolicy::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://iam-meta-api.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::iam_policy::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::iam_policy::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `IAMPolicy` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `IAMPolicy` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct IAMPolicy {
    inner: std::sync::Arc<dyn super::stub::dynamic::IAMPolicy>,
}

impl IAMPolicy {
    /// Returns a builder for [IAMPolicy].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_iam_v1::client::IAMPolicy;
    /// let client = IAMPolicy::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::iam_policy::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::iam_policy::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::IAMPolicy + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::IAMPolicy>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::IAMPolicy> {
        super::transport::IAMPolicy::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::IAMPolicy> {
        Self::build_transport(conf).await.map(super::tracing::IAMPolicy::new)
    }

    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    pub fn set_iam_policy(&self) -> super::builder::iam_policy::SetIamPolicy
    {
        super::builder::iam_policy::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    pub fn get_iam_policy(&self) -> super::builder::iam_policy::GetIamPolicy
    {
        super::builder::iam_policy::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::iam_policy::TestIamPermissions
    {
        super::builder::iam_policy::TestIamPermissions::new(self.inner.clone())
    }
}
