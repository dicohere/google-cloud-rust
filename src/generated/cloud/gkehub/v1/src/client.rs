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

/// Implements a client for the GKE Hub.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_gkehub_v1::client::GkeHub;
/// let client = GkeHub::builder().build().await?;
/// // use `client` to make requests to the GKE Hub.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// The GKE Hub service handles the registration of many Kubernetes clusters to
/// Google Cloud, and the management of multi-cluster features over those
/// clusters.
///
/// The GKE Hub service operates on the following resources:
///
/// * [Membership][google.cloud.gkehub.v1.Membership]
/// * [Feature][google.cloud.gkehub.v1.Feature]
///
/// GKE Hub is currently available in the global region and all regions in
/// <https://cloud.google.com/compute/docs/regions-zones>. Feature is only
/// available in global region while membership is global region and all the
/// regions.
///
/// **Membership management may be non-trivial:** it is recommended to use one
/// of the Google-provided client libraries or tools where possible when working
/// with Membership resources.
///
/// [google.cloud.gkehub.v1.Feature]: crate::model::Feature
/// [google.cloud.gkehub.v1.Membership]: crate::model::Membership
///
/// # Configuration
///
/// To configure `GkeHub` use the `with_*` methods in the type returned
/// by [builder()][GkeHub::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://gkehub.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::gke_hub::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::gke_hub::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `GkeHub` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `GkeHub` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct GkeHub {
    inner: std::sync::Arc<dyn super::stub::dynamic::GkeHub>,
}

impl GkeHub {
    /// Returns a builder for [GkeHub].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_gkehub_v1::client::GkeHub;
    /// let client = GkeHub::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::gke_hub::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::gke_hub::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::GkeHub + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::GkeHub>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::GkeHub> {
        super::transport::GkeHub::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::GkeHub> {
        Self::build_transport(conf).await.map(super::tracing::GkeHub::new)
    }

    /// Lists Memberships in a given project and location.
    pub fn list_memberships(&self) -> super::builder::gke_hub::ListMemberships
    {
        super::builder::gke_hub::ListMemberships::new(self.inner.clone())
    }

    /// Lists Features in a given project and location.
    pub fn list_features(&self) -> super::builder::gke_hub::ListFeatures
    {
        super::builder::gke_hub::ListFeatures::new(self.inner.clone())
    }

    /// Gets the details of a Membership.
    pub fn get_membership(&self) -> super::builder::gke_hub::GetMembership
    {
        super::builder::gke_hub::GetMembership::new(self.inner.clone())
    }

    /// Gets details of a single Feature.
    pub fn get_feature(&self) -> super::builder::gke_hub::GetFeature
    {
        super::builder::gke_hub::GetFeature::new(self.inner.clone())
    }

    /// Creates a new Membership.
    ///
    /// **This is currently only supported for GKE clusters on Google Cloud**.
    /// To register other clusters, follow the instructions at
    /// <https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster>.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_membership(&self) -> super::builder::gke_hub::CreateMembership
    {
        super::builder::gke_hub::CreateMembership::new(self.inner.clone())
    }

    /// Adds a new Feature.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_feature(&self) -> super::builder::gke_hub::CreateFeature
    {
        super::builder::gke_hub::CreateFeature::new(self.inner.clone())
    }

    /// Removes a Membership.
    ///
    /// **This is currently only supported for GKE clusters on Google Cloud**.
    /// To unregister other clusters, follow the instructions at
    /// <https://cloud.google.com/anthos/multicluster-management/connect/unregistering-a-cluster>.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_membership(&self) -> super::builder::gke_hub::DeleteMembership
    {
        super::builder::gke_hub::DeleteMembership::new(self.inner.clone())
    }

    /// Removes a Feature.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_feature(&self) -> super::builder::gke_hub::DeleteFeature
    {
        super::builder::gke_hub::DeleteFeature::new(self.inner.clone())
    }

    /// Updates an existing Membership.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_membership(&self) -> super::builder::gke_hub::UpdateMembership
    {
        super::builder::gke_hub::UpdateMembership::new(self.inner.clone())
    }

    /// Updates an existing Feature.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_feature(&self) -> super::builder::gke_hub::UpdateFeature
    {
        super::builder::gke_hub::UpdateFeature::new(self.inner.clone())
    }

    /// Generates the manifest for deployment of the GKE connect agent.
    ///
    /// **This method is used internally by Google-provided libraries.**
    /// Most clients should not need to call this method directly.
    pub fn generate_connect_manifest(&self) -> super::builder::gke_hub::GenerateConnectManifest
    {
        super::builder::gke_hub::GenerateConnectManifest::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::gke_hub::ListOperations
    {
        super::builder::gke_hub::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::gke_hub::GetOperation
    {
        super::builder::gke_hub::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::gke_hub::DeleteOperation
    {
        super::builder::gke_hub::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::gke_hub::CancelOperation
    {
        super::builder::gke_hub::CancelOperation::new(self.inner.clone())
    }
}
