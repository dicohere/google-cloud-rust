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

/// Implements a client for the Advisory Notifications API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_advisorynotifications_v1::client::AdvisoryNotificationsService;
/// let client = AdvisoryNotificationsService::builder().build().await?;
/// // use `client` to make requests to the Advisory Notifications API.
/// # gax::client_builder::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service to manage Security and Privacy Notifications.
///
/// # Configuration
///
/// To configure `AdvisoryNotificationsService` use the `with_*` methods in the type returned
/// by [builder()][AdvisoryNotificationsService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://advisorynotifications.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::advisory_notifications_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::advisory_notifications_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `AdvisoryNotificationsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AdvisoryNotificationsService` in
/// an [Rc](std::rc::Rc) or [Arc](std::sync::Arc) to reuse it, because it
/// already uses an `Arc` internally.
#[derive(Clone, Debug)]
pub struct AdvisoryNotificationsService {
    inner: std::sync::Arc<dyn super::stub::dynamic::AdvisoryNotificationsService>,
}

impl AdvisoryNotificationsService {
    /// Returns a builder for [AdvisoryNotificationsService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_advisorynotifications_v1::client::AdvisoryNotificationsService;
    /// let client = AdvisoryNotificationsService::builder().build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::advisory_notifications_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::advisory_notifications_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where T: super::stub::AdvisoryNotificationsService + 'static {
        Self { inner: std::sync::Arc::new(stub) }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<std::sync::Arc<dyn super::stub::dynamic::AdvisoryNotificationsService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(std::sync::Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(std::sync::Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::AdvisoryNotificationsService> {
        super::transport::AdvisoryNotificationsService::new(conf).await
    }

    async fn build_with_tracing(conf: gaxi::options::ClientConfig) -> gax::client_builder::Result<impl super::stub::AdvisoryNotificationsService> {
        Self::build_transport(conf).await.map(super::tracing::AdvisoryNotificationsService::new)
    }

    /// Lists notifications under a given parent.
    pub fn list_notifications(&self) -> super::builder::advisory_notifications_service::ListNotifications
    {
        super::builder::advisory_notifications_service::ListNotifications::new(self.inner.clone())
    }

    /// Gets a notification.
    pub fn get_notification(&self) -> super::builder::advisory_notifications_service::GetNotification
    {
        super::builder::advisory_notifications_service::GetNotification::new(self.inner.clone())
    }

    /// Get notification settings.
    pub fn get_settings(&self) -> super::builder::advisory_notifications_service::GetSettings
    {
        super::builder::advisory_notifications_service::GetSettings::new(self.inner.clone())
    }

    /// Update notification settings.
    pub fn update_settings(&self) -> super::builder::advisory_notifications_service::UpdateSettings
    {
        super::builder::advisory_notifications_service::UpdateSettings::new(self.inner.clone())
    }
}
