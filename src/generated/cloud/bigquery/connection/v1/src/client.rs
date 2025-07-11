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

/// Implements a client for the BigQuery Connection API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_bigquery_connection_v1::client::ConnectionService;
/// let client = ConnectionService::builder().build().await?;
/// // use `client` to make requests to the BigQuery Connection API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Manages external data source connections and credentials.
///
/// # Configuration
///
/// To configure `ConnectionService` use the `with_*` methods in the type returned
/// by [builder()][ConnectionService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://bigqueryconnection.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::connection_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::connection_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `ConnectionService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ConnectionService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct ConnectionService {
    inner: std::sync::Arc<dyn super::stub::dynamic::ConnectionService>,
}

impl ConnectionService {
    /// Returns a builder for [ConnectionService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_bigquery_connection_v1::client::ConnectionService;
    /// let client = ConnectionService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::connection_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::connection_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::ConnectionService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::ConnectionService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::ConnectionService> {
        super::transport::ConnectionService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::ConnectionService> {
        Self::build_transport(conf).await.map(super::tracing::ConnectionService::new)
    }

    /// Creates a new connection.
    pub fn create_connection(&self) -> super::builder::connection_service::CreateConnection
    {
        super::builder::connection_service::CreateConnection::new(self.inner.clone())
    }

    /// Returns specified connection.
    pub fn get_connection(&self) -> super::builder::connection_service::GetConnection
    {
        super::builder::connection_service::GetConnection::new(self.inner.clone())
    }

    /// Returns a list of connections in the given project.
    pub fn list_connections(&self) -> super::builder::connection_service::ListConnections
    {
        super::builder::connection_service::ListConnections::new(self.inner.clone())
    }

    /// Updates the specified connection. For security reasons, also resets
    /// credential if connection properties are in the update field mask.
    pub fn update_connection(&self) -> super::builder::connection_service::UpdateConnection
    {
        super::builder::connection_service::UpdateConnection::new(self.inner.clone())
    }

    /// Deletes connection and associated credential.
    pub fn delete_connection(&self) -> super::builder::connection_service::DeleteConnection
    {
        super::builder::connection_service::DeleteConnection::new(self.inner.clone())
    }

    /// Gets the access control policy for a resource.
    /// Returns an empty policy if the resource exists and does not have a policy
    /// set.
    pub fn get_iam_policy(&self) -> super::builder::connection_service::GetIamPolicy
    {
        super::builder::connection_service::GetIamPolicy::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified resource. Replaces any
    /// existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    pub fn set_iam_policy(&self) -> super::builder::connection_service::SetIamPolicy
    {
        super::builder::connection_service::SetIamPolicy::new(self.inner.clone())
    }

    /// Returns permissions that a caller has on the specified resource.
    /// If the resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::connection_service::TestIamPermissions
    {
        super::builder::connection_service::TestIamPermissions::new(self.inner.clone())
    }
}
