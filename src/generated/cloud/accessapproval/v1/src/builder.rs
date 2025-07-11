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

pub mod access_approval {
    use crate::Result;

    /// A builder for [AccessApproval][crate::client::AccessApproval].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_accessapproval_v1::*;
    /// # use builder::access_approval::ClientBuilder;
    /// # use client::AccessApproval;
    /// let builder : ClientBuilder = AccessApproval::builder();
    /// let client = builder
    ///     .with_endpoint("https://accessapproval.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::AccessApproval;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = AccessApproval;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::AccessApproval] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where R: std::default::Default {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [AccessApproval::list_approval_requests][crate::client::AccessApproval::list_approval_requests] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::ListApprovalRequests;
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
    /// fn prepare_request_builder() -> ListApprovalRequests {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct ListApprovalRequests(RequestBuilder<crate::model::ListApprovalRequestsMessage>);

    impl ListApprovalRequests {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListApprovalRequestsMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListApprovalRequestsResponse> {
            (*self.0.stub).list_approval_requests(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Streams each page in the collection.
        pub fn by_page(self) -> impl gax::paginator::Paginator<crate::model::ListApprovalRequestsResponse, gax::error::Error> {
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
        pub fn by_item(self) -> impl gax::paginator::ItemPaginator<crate::model::ListApprovalRequestsResponse, gax::error::Error> {
            use gax::paginator::Paginator;
            self.by_page().items()
        }

        /// Sets the value of [parent][crate::model::ListApprovalRequestsMessage::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListApprovalRequestsMessage::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListApprovalRequestsMessage::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListApprovalRequestsMessage::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListApprovalRequests {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::get_approval_request][crate::client::AccessApproval::get_approval_request] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::GetApprovalRequest;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetApprovalRequest {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetApprovalRequest(RequestBuilder<crate::model::GetApprovalRequestMessage>);

    impl GetApprovalRequest {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetApprovalRequestMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ApprovalRequest> {
            (*self.0.stub).get_approval_request(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetApprovalRequestMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetApprovalRequest {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::approve_approval_request][crate::client::AccessApproval::approve_approval_request] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::ApproveApprovalRequest;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> ApproveApprovalRequest {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct ApproveApprovalRequest(RequestBuilder<crate::model::ApproveApprovalRequestMessage>);

    impl ApproveApprovalRequest {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ApproveApprovalRequestMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ApprovalRequest> {
            (*self.0.stub).approve_approval_request(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::ApproveApprovalRequestMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [expire_time][crate::model::ApproveApprovalRequestMessage::expire_time].
        pub fn set_expire_time<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::Timestamp>
        {
            self.0.request.expire_time = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [expire_time][crate::model::ApproveApprovalRequestMessage::expire_time].
        pub fn set_or_clear_expire_time<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::Timestamp>
        {
            self.0.request.expire_time = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ApproveApprovalRequest {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::dismiss_approval_request][crate::client::AccessApproval::dismiss_approval_request] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::DismissApprovalRequest;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> DismissApprovalRequest {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct DismissApprovalRequest(RequestBuilder<crate::model::DismissApprovalRequestMessage>);

    impl DismissApprovalRequest {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DismissApprovalRequestMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ApprovalRequest> {
            (*self.0.stub).dismiss_approval_request(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::DismissApprovalRequestMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DismissApprovalRequest {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::invalidate_approval_request][crate::client::AccessApproval::invalidate_approval_request] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::InvalidateApprovalRequest;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> InvalidateApprovalRequest {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct InvalidateApprovalRequest(RequestBuilder<crate::model::InvalidateApprovalRequestMessage>);

    impl InvalidateApprovalRequest {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::InvalidateApprovalRequestMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ApprovalRequest> {
            (*self.0.stub).invalidate_approval_request(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::InvalidateApprovalRequestMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for InvalidateApprovalRequest {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::get_access_approval_settings][crate::client::AccessApproval::get_access_approval_settings] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::GetAccessApprovalSettings;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetAccessApprovalSettings {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetAccessApprovalSettings(RequestBuilder<crate::model::GetAccessApprovalSettingsMessage>);

    impl GetAccessApprovalSettings {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetAccessApprovalSettingsMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AccessApprovalSettings> {
            (*self.0.stub).get_access_approval_settings(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetAccessApprovalSettingsMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetAccessApprovalSettings {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::update_access_approval_settings][crate::client::AccessApproval::update_access_approval_settings] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::UpdateAccessApprovalSettings;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> UpdateAccessApprovalSettings {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct UpdateAccessApprovalSettings(RequestBuilder<crate::model::UpdateAccessApprovalSettingsMessage>);

    impl UpdateAccessApprovalSettings {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateAccessApprovalSettingsMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AccessApprovalSettings> {
            (*self.0.stub).update_access_approval_settings(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [settings][crate::model::UpdateAccessApprovalSettingsMessage::settings].
        pub fn set_settings<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::AccessApprovalSettings>
        {
            self.0.request.settings = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [settings][crate::model::UpdateAccessApprovalSettingsMessage::settings].
        pub fn set_or_clear_settings<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::AccessApprovalSettings>
        {
            self.0.request.settings = v.map(|x| x.into());
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateAccessApprovalSettingsMessage::update_mask].
        pub fn set_update_mask<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::FieldMask>
        {
            self.0.request.update_mask = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [update_mask][crate::model::UpdateAccessApprovalSettingsMessage::update_mask].
        pub fn set_or_clear_update_mask<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::FieldMask>
        {
            self.0.request.update_mask = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateAccessApprovalSettings {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::delete_access_approval_settings][crate::client::AccessApproval::delete_access_approval_settings] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::DeleteAccessApprovalSettings;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> DeleteAccessApprovalSettings {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct DeleteAccessApprovalSettings(RequestBuilder<crate::model::DeleteAccessApprovalSettingsMessage>);

    impl DeleteAccessApprovalSettings {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteAccessApprovalSettingsMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<()> {
            (*self.0.stub).delete_access_approval_settings(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::DeleteAccessApprovalSettingsMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for DeleteAccessApprovalSettings {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [AccessApproval::get_access_approval_service_account][crate::client::AccessApproval::get_access_approval_service_account] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_accessapproval_v1::builder;
    /// use builder::access_approval::GetAccessApprovalServiceAccount;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetAccessApprovalServiceAccount {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetAccessApprovalServiceAccount(RequestBuilder<crate::model::GetAccessApprovalServiceAccountMessage>);

    impl GetAccessApprovalServiceAccount {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::AccessApproval>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetAccessApprovalServiceAccountMessage>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AccessApprovalServiceAccount> {
            (*self.0.stub).get_access_approval_service_account(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetAccessApprovalServiceAccountMessage::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetAccessApprovalServiceAccount {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

}
