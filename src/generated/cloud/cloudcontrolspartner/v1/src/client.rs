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

/// Implements a client for the Cloud Controls Partner API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_cloudcontrolspartner_v1::client::CloudControlsPartnerCore;
/// let client = CloudControlsPartnerCore::builder().build().await?;
/// // use `client` to make requests to the Cloud Controls Partner API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// To configure `CloudControlsPartnerCore` use the `with_*` methods in the type returned
/// by [builder()][CloudControlsPartnerCore::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudcontrolspartner.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_controls_partner_core::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_controls_partner_core::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudControlsPartnerCore` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudControlsPartnerCore` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CloudControlsPartnerCore {
    inner: std::sync::Arc<dyn super::stub::dynamic::CloudControlsPartnerCore>,
}

impl CloudControlsPartnerCore {
    /// Returns a builder for [CloudControlsPartnerCore].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_cloudcontrolspartner_v1::client::CloudControlsPartnerCore;
    /// let client = CloudControlsPartnerCore::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_controls_partner_core::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::cloud_controls_partner_core::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CloudControlsPartnerCore + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CloudControlsPartnerCore>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudControlsPartnerCore> {
        super::transport::CloudControlsPartnerCore::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudControlsPartnerCore> {
        Self::build_transport(conf).await.map(super::tracing::CloudControlsPartnerCore::new)
    }

    /// Gets details of a single workload
    pub fn get_workload(&self) -> super::builder::cloud_controls_partner_core::GetWorkload
    {
        super::builder::cloud_controls_partner_core::GetWorkload::new(self.inner.clone())
    }

    /// Lists customer workloads for a given customer org id
    pub fn list_workloads(&self) -> super::builder::cloud_controls_partner_core::ListWorkloads
    {
        super::builder::cloud_controls_partner_core::ListWorkloads::new(self.inner.clone())
    }

    /// Gets details of a single customer
    pub fn get_customer(&self) -> super::builder::cloud_controls_partner_core::GetCustomer
    {
        super::builder::cloud_controls_partner_core::GetCustomer::new(self.inner.clone())
    }

    /// Lists customers of a partner identified by its Google Cloud organization ID
    pub fn list_customers(&self) -> super::builder::cloud_controls_partner_core::ListCustomers
    {
        super::builder::cloud_controls_partner_core::ListCustomers::new(self.inner.clone())
    }

    /// Gets the EKM connections associated with a workload
    pub fn get_ekm_connections(&self) -> super::builder::cloud_controls_partner_core::GetEkmConnections
    {
        super::builder::cloud_controls_partner_core::GetEkmConnections::new(self.inner.clone())
    }

    /// Gets the partner permissions granted for a workload
    pub fn get_partner_permissions(&self) -> super::builder::cloud_controls_partner_core::GetPartnerPermissions
    {
        super::builder::cloud_controls_partner_core::GetPartnerPermissions::new(self.inner.clone())
    }

    /// Deprecated: Only returns access approval requests directly associated with
    /// an assured workload folder.
    #[deprecated]
    pub fn list_access_approval_requests(&self) -> super::builder::cloud_controls_partner_core::ListAccessApprovalRequests
    {
        super::builder::cloud_controls_partner_core::ListAccessApprovalRequests::new(self.inner.clone())
    }

    /// Get details of a Partner.
    pub fn get_partner(&self) -> super::builder::cloud_controls_partner_core::GetPartner
    {
        super::builder::cloud_controls_partner_core::GetPartner::new(self.inner.clone())
    }

    /// Creates a new customer.
    pub fn create_customer(&self) -> super::builder::cloud_controls_partner_core::CreateCustomer
    {
        super::builder::cloud_controls_partner_core::CreateCustomer::new(self.inner.clone())
    }

    /// Update details of a single customer
    pub fn update_customer(&self) -> super::builder::cloud_controls_partner_core::UpdateCustomer
    {
        super::builder::cloud_controls_partner_core::UpdateCustomer::new(self.inner.clone())
    }

    /// Delete details of a single customer
    pub fn delete_customer(&self) -> super::builder::cloud_controls_partner_core::DeleteCustomer
    {
        super::builder::cloud_controls_partner_core::DeleteCustomer::new(self.inner.clone())
    }
}

/// Implements a client for the Cloud Controls Partner API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_cloudcontrolspartner_v1::client::CloudControlsPartnerMonitoring;
/// let client = CloudControlsPartnerMonitoring::builder().build().await?;
/// // use `client` to make requests to the Cloud Controls Partner API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// To configure `CloudControlsPartnerMonitoring` use the `with_*` methods in the type returned
/// by [builder()][CloudControlsPartnerMonitoring::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudcontrolspartner.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::cloud_controls_partner_monitoring::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::cloud_controls_partner_monitoring::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CloudControlsPartnerMonitoring` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudControlsPartnerMonitoring` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct CloudControlsPartnerMonitoring {
    inner: std::sync::Arc<dyn super::stub::dynamic::CloudControlsPartnerMonitoring>,
}

impl CloudControlsPartnerMonitoring {
    /// Returns a builder for [CloudControlsPartnerMonitoring].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_cloudcontrolspartner_v1::client::CloudControlsPartnerMonitoring;
    /// let client = CloudControlsPartnerMonitoring::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::cloud_controls_partner_monitoring::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::cloud_controls_partner_monitoring::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::CloudControlsPartnerMonitoring + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::CloudControlsPartnerMonitoring>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudControlsPartnerMonitoring> {
        super::transport::CloudControlsPartnerMonitoring::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::CloudControlsPartnerMonitoring> {
        Self::build_transport(conf).await.map(super::tracing::CloudControlsPartnerMonitoring::new)
    }

    /// Lists Violations for a workload
    /// Callers may also choose to read across multiple Customers or for a single
    /// customer as per
    /// [AIP-159](https://google.aip.dev/159) by using '-' (the hyphen or dash
    /// character) as a wildcard character instead of {customer} & {workload}.
    /// Format:
    /// `organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}`
    pub fn list_violations(&self) -> super::builder::cloud_controls_partner_monitoring::ListViolations
    {
        super::builder::cloud_controls_partner_monitoring::ListViolations::new(self.inner.clone())
    }

    /// Gets details of a single Violation.
    pub fn get_violation(&self) -> super::builder::cloud_controls_partner_monitoring::GetViolation
    {
        super::builder::cloud_controls_partner_monitoring::GetViolation::new(self.inner.clone())
    }
}
