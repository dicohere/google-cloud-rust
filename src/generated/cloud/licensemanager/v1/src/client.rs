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

/// Implements a client for the License Manager API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_licensemanager_v1::client::LicenseManager;
/// let client = LicenseManager::builder().build().await?;
/// // use `client` to make requests to the License Manager API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// To configure `LicenseManager` use the `with_*` methods in the type returned
/// by [builder()][LicenseManager::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://licensemanager.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::license_manager::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::license_manager::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `LicenseManager` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `LicenseManager` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct LicenseManager {
    inner: std::sync::Arc<dyn super::stub::dynamic::LicenseManager>,
}

impl LicenseManager {
    /// Returns a builder for [LicenseManager].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_licensemanager_v1::client::LicenseManager;
    /// let client = LicenseManager::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::license_manager::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::license_manager::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::LicenseManager + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::LicenseManager>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::LicenseManager> {
        super::transport::LicenseManager::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::LicenseManager> {
        Self::build_transport(conf).await.map(super::tracing::LicenseManager::new)
    }

    /// Lists Configurations in a given project and location.
    pub fn list_configurations(&self) -> super::builder::license_manager::ListConfigurations
    {
        super::builder::license_manager::ListConfigurations::new(self.inner.clone())
    }

    /// Gets details of a single Configuration.
    pub fn get_configuration(&self) -> super::builder::license_manager::GetConfiguration
    {
        super::builder::license_manager::GetConfiguration::new(self.inner.clone())
    }

    /// Creates a new Configuration in a given project and location.
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
    pub fn create_configuration(&self) -> super::builder::license_manager::CreateConfiguration
    {
        super::builder::license_manager::CreateConfiguration::new(self.inner.clone())
    }

    /// Updates the parameters of a single Configuration.
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
    pub fn update_configuration(&self) -> super::builder::license_manager::UpdateConfiguration
    {
        super::builder::license_manager::UpdateConfiguration::new(self.inner.clone())
    }

    /// Deletes a single Configuration.
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
    pub fn delete_configuration(&self) -> super::builder::license_manager::DeleteConfiguration
    {
        super::builder::license_manager::DeleteConfiguration::new(self.inner.clone())
    }

    /// Lists Instances in a given project and location.
    pub fn list_instances(&self) -> super::builder::license_manager::ListInstances
    {
        super::builder::license_manager::ListInstances::new(self.inner.clone())
    }

    /// Gets details of a single Instance.
    pub fn get_instance(&self) -> super::builder::license_manager::GetInstance
    {
        super::builder::license_manager::GetInstance::new(self.inner.clone())
    }

    /// Deactivates the given configuration.
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
    pub fn deactivate_configuration(&self) -> super::builder::license_manager::DeactivateConfiguration
    {
        super::builder::license_manager::DeactivateConfiguration::new(self.inner.clone())
    }

    /// Reactivates the given configuration.
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
    pub fn reactivate_configuration(&self) -> super::builder::license_manager::ReactivateConfiguration
    {
        super::builder::license_manager::ReactivateConfiguration::new(self.inner.clone())
    }

    /// License Usage information for a Configuration.
    pub fn query_configuration_license_usage(&self) -> super::builder::license_manager::QueryConfigurationLicenseUsage
    {
        super::builder::license_manager::QueryConfigurationLicenseUsage::new(self.inner.clone())
    }

    /// Aggregates Usage per Instance for a Configuration.
    pub fn aggregate_usage(&self) -> super::builder::license_manager::AggregateUsage
    {
        super::builder::license_manager::AggregateUsage::new(self.inner.clone())
    }

    /// Lists Products in a given project and location.
    pub fn list_products(&self) -> super::builder::license_manager::ListProducts
    {
        super::builder::license_manager::ListProducts::new(self.inner.clone())
    }

    /// Gets details of a single Product.
    pub fn get_product(&self) -> super::builder::license_manager::GetProduct
    {
        super::builder::license_manager::GetProduct::new(self.inner.clone())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::license_manager::ListLocations
    {
        super::builder::license_manager::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::license_manager::GetLocation
    {
        super::builder::license_manager::GetLocation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::license_manager::ListOperations
    {
        super::builder::license_manager::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::license_manager::GetOperation
    {
        super::builder::license_manager::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::license_manager::DeleteOperation
    {
        super::builder::license_manager::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::license_manager::CancelOperation
    {
        super::builder::license_manager::CancelOperation::new(self.inner.clone())
    }
}
