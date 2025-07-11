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

/// Implements a client for the Connectors API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_connectors_v1::client::Connectors;
/// let client = Connectors::builder().build().await?;
/// // use `client` to make requests to the Connectors API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Connectors is the interface for managing Connectors.
///
/// # Configuration
///
/// To configure `Connectors` use the `with_*` methods in the type returned
/// by [builder()][Connectors::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://connectors.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::connectors::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::connectors::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `Connectors` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Connectors` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct Connectors {
    inner: std::sync::Arc<dyn super::stub::dynamic::Connectors>,
}

impl Connectors {
    /// Returns a builder for [Connectors].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_connectors_v1::client::Connectors;
    /// let client = Connectors::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::connectors::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::connectors::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::Connectors + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::Connectors>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::Connectors> {
        super::transport::Connectors::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::Connectors> {
        Self::build_transport(conf).await.map(super::tracing::Connectors::new)
    }

    /// Lists Connections in a given project and location.
    pub fn list_connections(&self) -> super::builder::connectors::ListConnections
    {
        super::builder::connectors::ListConnections::new(self.inner.clone())
    }

    /// Gets details of a single Connection.
    pub fn get_connection(&self) -> super::builder::connectors::GetConnection
    {
        super::builder::connectors::GetConnection::new(self.inner.clone())
    }

    /// Creates a new Connection in a given project and location.
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
    pub fn create_connection(&self) -> super::builder::connectors::CreateConnection
    {
        super::builder::connectors::CreateConnection::new(self.inner.clone())
    }

    /// Updates the parameters of a single Connection.
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
    pub fn update_connection(&self) -> super::builder::connectors::UpdateConnection
    {
        super::builder::connectors::UpdateConnection::new(self.inner.clone())
    }

    /// Deletes a single Connection.
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
    pub fn delete_connection(&self) -> super::builder::connectors::DeleteConnection
    {
        super::builder::connectors::DeleteConnection::new(self.inner.clone())
    }

    /// Lists Providers in a given project and location.
    pub fn list_providers(&self) -> super::builder::connectors::ListProviders
    {
        super::builder::connectors::ListProviders::new(self.inner.clone())
    }

    /// Gets details of a provider.
    pub fn get_provider(&self) -> super::builder::connectors::GetProvider
    {
        super::builder::connectors::GetProvider::new(self.inner.clone())
    }

    /// Lists Connectors in a given project and location.
    pub fn list_connectors(&self) -> super::builder::connectors::ListConnectors
    {
        super::builder::connectors::ListConnectors::new(self.inner.clone())
    }

    /// Gets details of a single Connector.
    pub fn get_connector(&self) -> super::builder::connectors::GetConnector
    {
        super::builder::connectors::GetConnector::new(self.inner.clone())
    }

    /// Lists Connector Versions in a given project and location.
    pub fn list_connector_versions(&self) -> super::builder::connectors::ListConnectorVersions
    {
        super::builder::connectors::ListConnectorVersions::new(self.inner.clone())
    }

    /// Gets details of a single connector version.
    pub fn get_connector_version(&self) -> super::builder::connectors::GetConnectorVersion
    {
        super::builder::connectors::GetConnectorVersion::new(self.inner.clone())
    }

    /// Gets schema metadata of a connection.
    /// SchemaMetadata is a singleton resource for each connection.
    pub fn get_connection_schema_metadata(&self) -> super::builder::connectors::GetConnectionSchemaMetadata
    {
        super::builder::connectors::GetConnectionSchemaMetadata::new(self.inner.clone())
    }

    /// Refresh runtime schema of a connection.
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
    pub fn refresh_connection_schema_metadata(&self) -> super::builder::connectors::RefreshConnectionSchemaMetadata
    {
        super::builder::connectors::RefreshConnectionSchemaMetadata::new(self.inner.clone())
    }

    /// List schema of a runtime entities filtered by entity name.
    pub fn list_runtime_entity_schemas(&self) -> super::builder::connectors::ListRuntimeEntitySchemas
    {
        super::builder::connectors::ListRuntimeEntitySchemas::new(self.inner.clone())
    }

    /// List schema of a runtime actions filtered by action name.
    pub fn list_runtime_action_schemas(&self) -> super::builder::connectors::ListRuntimeActionSchemas
    {
        super::builder::connectors::ListRuntimeActionSchemas::new(self.inner.clone())
    }

    /// Gets the runtimeConfig of a location.
    /// RuntimeConfig is a singleton resource for each location.
    pub fn get_runtime_config(&self) -> super::builder::connectors::GetRuntimeConfig
    {
        super::builder::connectors::GetRuntimeConfig::new(self.inner.clone())
    }

    /// GetGlobalSettings gets settings of a project.
    /// GlobalSettings is a singleton resource.
    pub fn get_global_settings(&self) -> super::builder::connectors::GetGlobalSettings
    {
        super::builder::connectors::GetGlobalSettings::new(self.inner.clone())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::connectors::ListLocations
    {
        super::builder::connectors::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::connectors::GetLocation
    {
        super::builder::connectors::GetLocation::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(&self) -> super::builder::connectors::SetIamPolicy
    {
        super::builder::connectors::SetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(&self) -> super::builder::connectors::GetIamPolicy
    {
        super::builder::connectors::GetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::connectors::TestIamPermissions
    {
        super::builder::connectors::TestIamPermissions::new(self.inner.clone())
    }

    /// Lists operations that match the specified filter in the request. If
    /// the server doesn't support this method, it returns `UNIMPLEMENTED`.
    pub fn list_operations(&self) -> super::builder::connectors::ListOperations
    {
        super::builder::connectors::ListOperations::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(&self) -> super::builder::connectors::GetOperation
    {
        super::builder::connectors::GetOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(&self) -> super::builder::connectors::DeleteOperation
    {
        super::builder::connectors::DeleteOperation::new(self.inner.clone())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(&self) -> super::builder::connectors::CancelOperation
    {
        super::builder::connectors::CancelOperation::new(self.inner.clone())
    }
}
