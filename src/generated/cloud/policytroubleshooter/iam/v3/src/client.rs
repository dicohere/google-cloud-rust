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

/// Implements a client for the Policy Troubleshooter API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_policytroubleshooter_iam_v3::client::PolicyTroubleshooter;
/// let client = PolicyTroubleshooter::builder().build().await?;
/// // use `client` to make requests to the Policy Troubleshooter API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// IAM Policy Troubleshooter service.
///
/// This service helps you troubleshoot access issues for Google Cloud resources.
///
/// # Configuration
///
/// To configure `PolicyTroubleshooter` use the `with_*` methods in the type returned
/// by [builder()][PolicyTroubleshooter::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://policytroubleshooter.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::policy_troubleshooter::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::policy_troubleshooter::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `PolicyTroubleshooter` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `PolicyTroubleshooter` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct PolicyTroubleshooter {
    inner: std::sync::Arc<dyn super::stub::dynamic::PolicyTroubleshooter>,
}

impl PolicyTroubleshooter {
    /// Returns a builder for [PolicyTroubleshooter].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_policytroubleshooter_iam_v3::client::PolicyTroubleshooter;
    /// let client = PolicyTroubleshooter::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::policy_troubleshooter::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::policy_troubleshooter::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::PolicyTroubleshooter + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::PolicyTroubleshooter>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::PolicyTroubleshooter> {
        super::transport::PolicyTroubleshooter::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::PolicyTroubleshooter> {
        Self::build_transport(conf).await.map(super::tracing::PolicyTroubleshooter::new)
    }

    /// Checks whether a principal has a specific permission for a specific
    /// resource, and explains why the principal does or doesn't have that
    /// permission.
    pub fn troubleshoot_iam_policy(&self) -> super::builder::policy_troubleshooter::TroubleshootIamPolicy
    {
        super::builder::policy_troubleshooter::TroubleshootIamPolicy::new(self.inner.clone())
    }
}
