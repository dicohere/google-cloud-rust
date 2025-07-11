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

/// Implements a client for the Managed Service for Microsoft Active Directory API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_managedidentities_v1::client::ManagedIdentitiesService;
/// let client = ManagedIdentitiesService::builder().build().await?;
/// // use `client` to make requests to the Managed Service for Microsoft Active Directory API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// API Overview
///
/// The `managedidentites.googleapis.com` service implements the Google Cloud
/// Managed Identites API for identity services
/// (e.g. Microsoft Active Directory).
///
/// The Managed Identities service provides methods to manage
/// (create/read/update/delete) domains, reset managed identities admin password,
/// add/remove domain controllers in GCP regions and add/remove VPC peering.
///
/// Data Model
///
/// The Managed Identities service exposes the following resources:
///
/// * Locations as global, named as follows:
///   `projects/{project_id}/locations/global`.
///
/// * Domains, named as follows:
///   `/projects/{project_id}/locations/global/domain/{domain_name}`.
///
///
/// The `{domain_name}` refers to fully qualified domain name in the customer
/// project e.g. mydomain.myorganization.com, with the following restrictions:
///
/// * Must contain only lowercase letters, numbers, periods and hyphens.
/// * Must start with a letter.
/// * Must contain between 2-64 characters.
/// * Must end with a number or a letter.
/// * Must not start with period.
/// * First segement length (mydomain form example above) shouldn't exceed
///   15 chars.
/// * The last segment cannot be fully numeric.
/// * Must be unique within the customer project.
///
/// # Configuration
///
/// To configure `ManagedIdentitiesService` use the `with_*` methods in the type returned
/// by [builder()][ManagedIdentitiesService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://managedidentities.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::managed_identities_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::managed_identities_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `ManagedIdentitiesService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ManagedIdentitiesService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct ManagedIdentitiesService {
    inner: std::sync::Arc<dyn super::stub::dynamic::ManagedIdentitiesService>,
}

impl ManagedIdentitiesService {
    /// Returns a builder for [ManagedIdentitiesService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_managedidentities_v1::client::ManagedIdentitiesService;
    /// let client = ManagedIdentitiesService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::managed_identities_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::managed_identities_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::ManagedIdentitiesService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::ManagedIdentitiesService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::ManagedIdentitiesService> {
        super::transport::ManagedIdentitiesService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::ManagedIdentitiesService> {
        Self::build_transport(conf).await.map(super::tracing::ManagedIdentitiesService::new)
    }

    /// Creates a Microsoft AD domain.
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
    pub fn create_microsoft_ad_domain(&self) -> super::builder::managed_identities_service::CreateMicrosoftAdDomain
    {
        super::builder::managed_identities_service::CreateMicrosoftAdDomain::new(self.inner.clone())
    }

    /// Resets a domain's administrator password.
    pub fn reset_admin_password(&self) -> super::builder::managed_identities_service::ResetAdminPassword
    {
        super::builder::managed_identities_service::ResetAdminPassword::new(self.inner.clone())
    }

    /// Lists domains in a project.
    pub fn list_domains(&self) -> super::builder::managed_identities_service::ListDomains
    {
        super::builder::managed_identities_service::ListDomains::new(self.inner.clone())
    }

    /// Gets information about a domain.
    pub fn get_domain(&self) -> super::builder::managed_identities_service::GetDomain
    {
        super::builder::managed_identities_service::GetDomain::new(self.inner.clone())
    }

    /// Updates the metadata and configuration of a domain.
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
    pub fn update_domain(&self) -> super::builder::managed_identities_service::UpdateDomain
    {
        super::builder::managed_identities_service::UpdateDomain::new(self.inner.clone())
    }

    /// Deletes a domain.
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
    pub fn delete_domain(&self) -> super::builder::managed_identities_service::DeleteDomain
    {
        super::builder::managed_identities_service::DeleteDomain::new(self.inner.clone())
    }

    /// Adds an AD trust to a domain.
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
    pub fn attach_trust(&self) -> super::builder::managed_identities_service::AttachTrust
    {
        super::builder::managed_identities_service::AttachTrust::new(self.inner.clone())
    }

    /// Updates the DNS conditional forwarder.
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
    pub fn reconfigure_trust(&self) -> super::builder::managed_identities_service::ReconfigureTrust
    {
        super::builder::managed_identities_service::ReconfigureTrust::new(self.inner.clone())
    }

    /// Removes an AD trust.
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
    pub fn detach_trust(&self) -> super::builder::managed_identities_service::DetachTrust
    {
        super::builder::managed_identities_service::DetachTrust::new(self.inner.clone())
    }

    /// Validates a trust state, that the target domain is reachable, and that the
    /// target domain is able to accept incoming trust requests.
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
    pub fn validate_trust(&self) -> super::builder::managed_identities_service::ValidateTrust
    {
        super::builder::managed_identities_service::ValidateTrust::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::managed_identities_service::ListOperations
    {
        super::builder::managed_identities_service::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::managed_identities_service::GetOperation
    {
        super::builder::managed_identities_service::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::managed_identities_service::DeleteOperation
    {
        super::builder::managed_identities_service::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::managed_identities_service::CancelOperation
    {
        super::builder::managed_identities_service::CancelOperation::new(self.inner.clone())
    }
}
