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

/// Implements a client for the Secret Manager API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use secretmanager_openapi_v1::client::SecretManagerService;
/// let client = SecretManagerService::builder().build().await?;
/// // use `client` to make requests to the Secret Manager API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Stores sensitive data such as API keys, passwords, and certificates.
/// Provides convenience while improving security.
///
/// # Configuration
///
/// To configure `SecretManagerService` use the `with_*` methods in the type returned
/// by [builder()][SecretManagerService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://secretmanager.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::secret_manager_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::secret_manager_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `SecretManagerService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `SecretManagerService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct SecretManagerService {
    inner: std::sync::Arc<dyn super::stub::dynamic::SecretManagerService>,
}

impl SecretManagerService {
    /// Returns a builder for [SecretManagerService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use secretmanager_openapi_v1::client::SecretManagerService;
    /// let client = SecretManagerService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::secret_manager_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::secret_manager_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::SecretManagerService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::SecretManagerService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::SecretManagerService> {
        super::transport::SecretManagerService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::SecretManagerService> {
        Self::build_transport(conf).await.map(super::tracing::SecretManagerService::new)
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(&self) -> super::builder::secret_manager_service::ListLocations
    {
        super::builder::secret_manager_service::ListLocations::new(self.inner.clone())
    }

    /// Gets information about a location.
    pub fn get_location(&self) -> super::builder::secret_manager_service::GetLocation
    {
        super::builder::secret_manager_service::GetLocation::new(self.inner.clone())
    }

    /// Lists Secrets.
    pub fn list_secrets(&self) -> super::builder::secret_manager_service::ListSecrets
    {
        super::builder::secret_manager_service::ListSecrets::new(self.inner.clone())
    }

    /// Creates a new Secret containing no SecretVersions.
    pub fn create_secret(&self) -> super::builder::secret_manager_service::CreateSecret
    {
        super::builder::secret_manager_service::CreateSecret::new(self.inner.clone())
    }

    /// Lists Secrets.
    pub fn list_secrets_by_project_and_location(&self) -> super::builder::secret_manager_service::ListSecretsByProjectAndLocation
    {
        super::builder::secret_manager_service::ListSecretsByProjectAndLocation::new(self.inner.clone())
    }

    /// Creates a new Secret containing no SecretVersions.
    pub fn create_secret_by_project_and_location(&self) -> super::builder::secret_manager_service::CreateSecretByProjectAndLocation
    {
        super::builder::secret_manager_service::CreateSecretByProjectAndLocation::new(self.inner.clone())
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    pub fn add_secret_version(&self) -> super::builder::secret_manager_service::AddSecretVersion
    {
        super::builder::secret_manager_service::AddSecretVersion::new(self.inner.clone())
    }

    /// Creates a new SecretVersion containing secret data and attaches
    /// it to an existing Secret.
    pub fn add_secret_version_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::AddSecretVersionByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::AddSecretVersionByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Gets metadata for a given Secret.
    pub fn get_secret(&self) -> super::builder::secret_manager_service::GetSecret
    {
        super::builder::secret_manager_service::GetSecret::new(self.inner.clone())
    }

    /// Deletes a Secret.
    pub fn delete_secret(&self) -> super::builder::secret_manager_service::DeleteSecret
    {
        super::builder::secret_manager_service::DeleteSecret::new(self.inner.clone())
    }

    /// Updates metadata of an existing Secret.
    pub fn update_secret(&self) -> super::builder::secret_manager_service::UpdateSecret
    {
        super::builder::secret_manager_service::UpdateSecret::new(self.inner.clone())
    }

    /// Gets metadata for a given Secret.
    pub fn get_secret_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::GetSecretByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::GetSecretByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Deletes a Secret.
    pub fn delete_secret_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::DeleteSecretByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::DeleteSecretByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Updates metadata of an existing Secret.
    pub fn update_secret_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::UpdateSecretByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::UpdateSecretByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    pub fn list_secret_versions(&self) -> super::builder::secret_manager_service::ListSecretVersions
    {
        super::builder::secret_manager_service::ListSecretVersions::new(self.inner.clone())
    }

    /// Lists SecretVersions. This call does not return secret
    /// data.
    pub fn list_secret_versions_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::ListSecretVersionsByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::ListSecretVersionsByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn get_secret_version(&self) -> super::builder::secret_manager_service::GetSecretVersion
    {
        super::builder::secret_manager_service::GetSecretVersion::new(self.inner.clone())
    }

    /// Gets metadata for a SecretVersion.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn get_secret_version_by_project_and_location_and_secret_and_version(&self) -> super::builder::secret_manager_service::GetSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        super::builder::secret_manager_service::GetSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn access_secret_version(&self) -> super::builder::secret_manager_service::AccessSecretVersion
    {
        super::builder::secret_manager_service::AccessSecretVersion::new(self.inner.clone())
    }

    /// Accesses a SecretVersion. This call returns the secret data.
    ///
    /// `projects/_*_/secrets/_*_/versions/latest` is an alias to the most recently
    /// created SecretVersion.
    pub fn access_secret_version_by_project_and_location_and_secret_and_version(&self) -> super::builder::secret_manager_service::AccessSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        super::builder::secret_manager_service::AccessSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    pub fn disable_secret_version(&self) -> super::builder::secret_manager_service::DisableSecretVersion
    {
        super::builder::secret_manager_service::DisableSecretVersion::new(self.inner.clone())
    }

    /// Disables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DISABLED.
    pub fn disable_secret_version_by_project_and_location_and_secret_and_version(&self) -> super::builder::secret_manager_service::DisableSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        super::builder::secret_manager_service::DisableSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    pub fn enable_secret_version(&self) -> super::builder::secret_manager_service::EnableSecretVersion
    {
        super::builder::secret_manager_service::EnableSecretVersion::new(self.inner.clone())
    }

    /// Enables a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// ENABLED.
    pub fn enable_secret_version_by_project_and_location_and_secret_and_version(&self) -> super::builder::secret_manager_service::EnableSecretVersionByProjectAndLocationAndSecretAndVersion
    {
        super::builder::secret_manager_service::EnableSecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    pub fn destroy_secret_version(&self) -> super::builder::secret_manager_service::DestroySecretVersion
    {
        super::builder::secret_manager_service::DestroySecretVersion::new(self.inner.clone())
    }

    /// Destroys a SecretVersion.
    ///
    /// Sets the state of the SecretVersion to
    /// DESTROYED and irrevocably destroys the
    /// secret data.
    pub fn destroy_secret_version_by_project_and_location_and_secret_and_version(&self) -> super::builder::secret_manager_service::DestroySecretVersionByProjectAndLocationAndSecretAndVersion
    {
        super::builder::secret_manager_service::DestroySecretVersionByProjectAndLocationAndSecretAndVersion::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    pub fn set_iam_policy(&self) -> super::builder::secret_manager_service::SetIamPolicy
    {
        super::builder::secret_manager_service::SetIamPolicy::new(self.inner.clone())
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on SecretVersions are enforced according
    /// to the policy set on the associated Secret.
    pub fn set_iam_policy_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::SetIamPolicyByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::SetIamPolicyByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    pub fn get_iam_policy(&self) -> super::builder::secret_manager_service::GetIamPolicy
    {
        super::builder::secret_manager_service::GetIamPolicy::new(self.inner.clone())
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    pub fn get_iam_policy_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::GetIamPolicyByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::GetIamPolicyByProjectAndLocationAndSecret::new(self.inner.clone())
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub fn test_iam_permissions(&self) -> super::builder::secret_manager_service::TestIamPermissions
    {
        super::builder::secret_manager_service::TestIamPermissions::new(self.inner.clone())
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    pub fn test_iam_permissions_by_project_and_location_and_secret(&self) -> super::builder::secret_manager_service::TestIamPermissionsByProjectAndLocationAndSecret
    {
        super::builder::secret_manager_service::TestIamPermissionsByProjectAndLocationAndSecret::new(self.inner.clone())
    }
}
