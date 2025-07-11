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

/// Implements a client for the Cloud Memorystore for Memcached API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_memcache_v1::client::CloudMemcache;
/// let client = CloudMemcache::builder().build().await?;
/// // use `client` to make requests to the Cloud Memorystore for Memcached API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Configures and manages Cloud Memorystore for Memcached instances.
///
/// The `memcache.googleapis.com` service implements the Google Cloud Memorystore
/// for Memcached API and defines the following resource model for managing
/// Memorystore Memcached (also called Memcached below) instances:
///
/// * The service works with a collection of cloud projects, named: `/projects/*`
/// * Each project has a collection of available locations, named: `/locations/*`
/// * Each location has a collection of Memcached instances, named:
///   `/instances/*`
/// * As such, Memcached instances are resources of the form:
///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
///
/// Note that location_id must be a GCP `region`; for example:
///
/// * `projects/my-memcached-project/locations/us-central1/instances/my-memcached`
///
/// # Configuration
///
/// To configure `CloudMemcache` use the `with_*` methods in the type returned
/// by [builder()][CloudMemcache::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://memcache.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_memcache::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_memcache::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudMemcache` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudMemcache` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CloudMemcache {
    inner: std::sync::Arc<dyn super::stub::dynamic::CloudMemcache>,
}

impl CloudMemcache {
    /// Returns a builder for [CloudMemcache].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_memcache_v1::client::CloudMemcache;
    /// let client = CloudMemcache::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_memcache::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::cloud_memcache::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CloudMemcache + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CloudMemcache>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudMemcache> {
        super::transport::CloudMemcache::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudMemcache> {
        Self::build_transport(conf).await.map(super::tracing::CloudMemcache::new)
    }

    /// Lists Instances in a given location.
    pub fn list_instances(&self) -> super::builder::cloud_memcache::ListInstances
    {
        super::builder::cloud_memcache::ListInstances::new(self.inner.clone())
    }

    /// Gets details of a single Instance.
    pub fn get_instance(&self) -> super::builder::cloud_memcache::GetInstance
    {
        super::builder::cloud_memcache::GetInstance::new(self.inner.clone())
    }

    /// Creates a new Instance in a given location.
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
    pub fn create_instance(&self) -> super::builder::cloud_memcache::CreateInstance
    {
        super::builder::cloud_memcache::CreateInstance::new(self.inner.clone())
    }

    /// Updates an existing Instance in a given project and location.
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
    pub fn update_instance(&self) -> super::builder::cloud_memcache::UpdateInstance
    {
        super::builder::cloud_memcache::UpdateInstance::new(self.inner.clone())
    }

    /// Updates the defined Memcached parameters for an existing instance.
    /// This method only stages the parameters, it must be followed by
    /// `ApplyParameters` to apply the parameters to nodes of the Memcached
    /// instance.
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
    pub fn update_parameters(&self) -> super::builder::cloud_memcache::UpdateParameters
    {
        super::builder::cloud_memcache::UpdateParameters::new(self.inner.clone())
    }

    /// Deletes a single Instance.
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
    pub fn delete_instance(&self) -> super::builder::cloud_memcache::DeleteInstance
    {
        super::builder::cloud_memcache::DeleteInstance::new(self.inner.clone())
    }

    /// `ApplyParameters` restarts the set of specified nodes in order to update
    /// them to the current set of parameters for the Memcached Instance.
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
    pub fn apply_parameters(&self) -> super::builder::cloud_memcache::ApplyParameters
    {
        super::builder::cloud_memcache::ApplyParameters::new(self.inner.clone())
    }

    /// Reschedules upcoming maintenance event.
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
    pub fn reschedule_maintenance(&self) -> super::builder::cloud_memcache::RescheduleMaintenance
    {
        super::builder::cloud_memcache::RescheduleMaintenance::new(self.inner.clone())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::cloud_memcache::ListLocations
    {
        super::builder::cloud_memcache::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::cloud_memcache::GetLocation
    {
        super::builder::cloud_memcache::GetLocation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::cloud_memcache::ListOperations
    {
        super::builder::cloud_memcache::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::cloud_memcache::GetOperation
    {
        super::builder::cloud_memcache::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::cloud_memcache::DeleteOperation
    {
        super::builder::cloud_memcache::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::cloud_memcache::CancelOperation
    {
        super::builder::cloud_memcache::CancelOperation::new(self.inner.clone())
    }
}
