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

/// Implements a client for the BeyondCorp API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_beyondcorp_appconnections_v1::client::AppConnectionsService;
/// let client = AppConnectionsService::builder().build().await?;
/// // use `client` to make requests to the BeyondCorp API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// API Overview:
///
/// The `beyondcorp.googleapis.com` service implements the Google Cloud
/// BeyondCorp API.
///
/// Data Model:
///
/// The AppConnectionsService exposes the following resources:
///
/// * AppConnections, named as follows:
///   `projects/{project_id}/locations/{location_id}/appConnections/{app_connection_id}`.
///
/// The AppConnectionsService service provides methods to manage
/// (create/read/update/delete) BeyondCorp AppConnections.
///
/// # Configuration
///
/// To configure `AppConnectionsService` use the `with_*` methods in the type returned
/// by [builder()][AppConnectionsService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://beyondcorp.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::app_connections_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::app_connections_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `AppConnectionsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AppConnectionsService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct AppConnectionsService {
    inner: std::sync::Arc<dyn super::stub::dynamic::AppConnectionsService>,
}

impl AppConnectionsService {
    /// Returns a builder for [AppConnectionsService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_beyondcorp_appconnections_v1::client::AppConnectionsService;
    /// let client = AppConnectionsService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::app_connections_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::app_connections_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::AppConnectionsService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::AppConnectionsService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::AppConnectionsService> {
        super::transport::AppConnectionsService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::AppConnectionsService> {
        Self::build_transport(conf).await.map(super::tracing::AppConnectionsService::new)
    }

    /// Lists AppConnections in a given project and location.
    pub fn list_app_connections(&self) -> super::builder::app_connections_service::ListAppConnections
    {
        super::builder::app_connections_service::ListAppConnections::new(self.inner.clone())
    }

    /// Gets details of a single AppConnection.
    pub fn get_app_connection(&self) -> super::builder::app_connections_service::GetAppConnection
    {
        super::builder::app_connections_service::GetAppConnection::new(self.inner.clone())
    }

    /// Creates a new AppConnection in a given project and location.
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
    pub fn create_app_connection(&self) -> super::builder::app_connections_service::CreateAppConnection
    {
        super::builder::app_connections_service::CreateAppConnection::new(self.inner.clone())
    }

    /// Updates the parameters of a single AppConnection.
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
    pub fn update_app_connection(&self) -> super::builder::app_connections_service::UpdateAppConnection
    {
        super::builder::app_connections_service::UpdateAppConnection::new(self.inner.clone())
    }

    /// Deletes a single AppConnection.
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
    pub fn delete_app_connection(&self) -> super::builder::app_connections_service::DeleteAppConnection
    {
        super::builder::app_connections_service::DeleteAppConnection::new(self.inner.clone())
    }

    /// Resolves AppConnections details for a given AppConnector.
    /// An internal method called by a connector to find AppConnections to connect
    /// to.
    pub fn resolve_app_connections(&self) -> super::builder::app_connections_service::ResolveAppConnections
    {
        super::builder::app_connections_service::ResolveAppConnections::new(self.inner.clone())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::app_connections_service::ListLocations
    {
        super::builder::app_connections_service::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::app_connections_service::GetLocation
    {
        super::builder::app_connections_service::GetLocation::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(&self) -> super::builder::app_connections_service::SetIamPolicy
    {
        super::builder::app_connections_service::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(&self) -> super::builder::app_connections_service::GetIamPolicy
    {
        super::builder::app_connections_service::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::app_connections_service::TestIamPermissions
    {
        super::builder::app_connections_service::TestIamPermissions::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(&self) -> super::builder::app_connections_service::ListOperations
    {
        super::builder::app_connections_service::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::app_connections_service::GetOperation
    {
        super::builder::app_connections_service::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::app_connections_service::DeleteOperation
    {
        super::builder::app_connections_service::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::app_connections_service::CancelOperation
    {
        super::builder::app_connections_service::CancelOperation::new(self.inner.clone())
    }
}
