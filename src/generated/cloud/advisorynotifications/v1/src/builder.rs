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

pub mod advisory_notifications_service {
    use crate::Result;

    /// A builder for [AdvisoryNotificationsService][crate::client::AdvisoryNotificationsService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_advisorynotifications_v1::*;
    /// # use builder::advisory_notifications_service::ClientBuilder;
    /// # use client::AdvisoryNotificationsService;
    /// let builder : ClientBuilder = AdvisoryNotificationsService::builder();
    /// let client = builder
    ///     .with_endpoint("https://advisorynotifications.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::AdvisoryNotificationsService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = AdvisoryNotificationsService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::AdvisoryNotificationsService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::AdvisoryNotificationsService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where R: std::default::Default {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AdvisoryNotificationsService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [AdvisoryNotificationsService::list_notifications][crate::client::AdvisoryNotificationsService::list_notifications] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_advisorynotifications_v1::builder;
    /// use builder::advisory_notifications_service::ListNotifications;
    /// # tokio_test::block_on(async {
    /// use gax::paginator::ItemPaginator;
    ///
    /// let builder = prepare_request_builder();
    /// let mut items = builder.by_item();
    /// while let Some(result) = items.next().await {
    ///   let item = result?;
    /// }
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> ListNotifications {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct ListNotifications(RequestBuilder<crate::model::ListNotificationsRequest>);

    impl ListNotifications {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AdvisoryNotificationsService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListNotificationsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListNotificationsResponse> {
            (*self.0.stub).list_notifications(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Streams each page in the collection.
        pub fn by_page(self) -> impl gax::paginator::Paginator<crate::model::ListNotificationsResponse, gax::error::Error> {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::internal::new_paginator(token, execute)
        }

        /// Streams each item in the collection.
        pub fn by_item(self) -> impl gax::paginator::ItemPaginator<crate::model::ListNotificationsResponse, gax::error::Error> {
            use gax::paginator::Paginator;
            self.by_page().items()
        }

        /// Sets the value of [parent][crate::model::ListNotificationsRequest::parent].
        ///
        /// This is a **required** field for requests.
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListNotificationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListNotificationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [view][crate::model::ListNotificationsRequest::view].
        pub fn set_view<T: Into<crate::model::NotificationView>>(mut self, v: T) -> Self {
            self.0.request.view = v.into();
            self
        }

        /// Sets the value of [language_code][crate::model::ListNotificationsRequest::language_code].
        pub fn set_language_code<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.language_code = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListNotifications {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AdvisoryNotificationsService::get_notification][crate::client::AdvisoryNotificationsService::get_notification] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_advisorynotifications_v1::builder;
    /// use builder::advisory_notifications_service::GetNotification;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetNotification {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetNotification(RequestBuilder<crate::model::GetNotificationRequest>);

    impl GetNotification {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AdvisoryNotificationsService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetNotificationRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Notification> {
            (*self.0.stub).get_notification(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetNotificationRequest::name].
        ///
        /// This is a **required** field for requests.
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [language_code][crate::model::GetNotificationRequest::language_code].
        pub fn set_language_code<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.language_code = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetNotification {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AdvisoryNotificationsService::get_settings][crate::client::AdvisoryNotificationsService::get_settings] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_advisorynotifications_v1::builder;
    /// use builder::advisory_notifications_service::GetSettings;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetSettings {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetSettings(RequestBuilder<crate::model::GetSettingsRequest>);

    impl GetSettings {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AdvisoryNotificationsService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetSettingsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Settings> {
            (*self.0.stub).get_settings(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetSettingsRequest::name].
        ///
        /// This is a **required** field for requests.
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetSettings {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AdvisoryNotificationsService::update_settings][crate::client::AdvisoryNotificationsService::update_settings] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_advisorynotifications_v1::builder;
    /// use builder::advisory_notifications_service::UpdateSettings;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> UpdateSettings {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct UpdateSettings(RequestBuilder<crate::model::UpdateSettingsRequest>);

    impl UpdateSettings {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AdvisoryNotificationsService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateSettingsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Settings> {
            (*self.0.stub).update_settings(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [settings][crate::model::UpdateSettingsRequest::settings].
        ///
        /// This is a **required** field for requests.
        pub fn set_settings<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::Settings>
        {
            self.0.request.settings = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [settings][crate::model::UpdateSettingsRequest::settings].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_settings<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::Settings>
        {
            self.0.request.settings = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateSettings {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

}
