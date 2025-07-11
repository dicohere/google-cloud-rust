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

pub mod profiler_service {
    use crate::Result;

    /// A builder for [ProfilerService][crate::client::ProfilerService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_profiler_v2::*;
    /// # use builder::profiler_service::ClientBuilder;
    /// # use client::ProfilerService;
    /// let builder : ClientBuilder = ProfilerService::builder();
    /// let client = builder
    ///     .with_endpoint("https://cloudprofiler.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::ProfilerService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = ProfilerService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::ProfilerService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::ProfilerService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where R: std::default::Default {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::ProfilerService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [ProfilerService::create_profile][crate::client::ProfilerService::create_profile] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_profiler_v2::builder;
    /// use builder::profiler_service::CreateProfile;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> CreateProfile {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct CreateProfile(RequestBuilder<crate::model::CreateProfileRequest>);

    impl CreateProfile {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::ProfilerService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateProfileRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Profile> {
            (*self.0.stub).create_profile(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [parent][crate::model::CreateProfileRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [deployment][crate::model::CreateProfileRequest::deployment].
        pub fn set_deployment<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::Deployment>
        {
            self.0.request.deployment = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [deployment][crate::model::CreateProfileRequest::deployment].
        pub fn set_or_clear_deployment<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::Deployment>
        {
            self.0.request.deployment = v.map(|x| x.into());
            self
        }

        /// Sets the value of [profile_type][crate::model::CreateProfileRequest::profile_type].
        pub fn set_profile_type<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::ProfileType>
        {
            use std::iter::Iterator;
            self.0.request.profile_type = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ProfilerService::create_offline_profile][crate::client::ProfilerService::create_offline_profile] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_profiler_v2::builder;
    /// use builder::profiler_service::CreateOfflineProfile;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> CreateOfflineProfile {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct CreateOfflineProfile(RequestBuilder<crate::model::CreateOfflineProfileRequest>);

    impl CreateOfflineProfile {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::ProfilerService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateOfflineProfileRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Profile> {
            (*self.0.stub).create_offline_profile(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [parent][crate::model::CreateOfflineProfileRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [profile][crate::model::CreateOfflineProfileRequest::profile].
        pub fn set_profile<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::Profile>
        {
            self.0.request.profile = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [profile][crate::model::CreateOfflineProfileRequest::profile].
        pub fn set_or_clear_profile<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::Profile>
        {
            self.0.request.profile = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateOfflineProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [ProfilerService::update_profile][crate::client::ProfilerService::update_profile] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_profiler_v2::builder;
    /// use builder::profiler_service::UpdateProfile;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> UpdateProfile {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct UpdateProfile(RequestBuilder<crate::model::UpdateProfileRequest>);

    impl UpdateProfile {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::ProfilerService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::UpdateProfileRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Profile> {
            (*self.0.stub).update_profile(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [profile][crate::model::UpdateProfileRequest::profile].
        pub fn set_profile<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::Profile>
        {
            self.0.request.profile = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [profile][crate::model::UpdateProfileRequest::profile].
        pub fn set_or_clear_profile<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::Profile>
        {
            self.0.request.profile = v.map(|x| x.into());
            self
        }

        /// Sets the value of [update_mask][crate::model::UpdateProfileRequest::update_mask].
        pub fn set_update_mask<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::FieldMask>
        {
            self.0.request.update_mask = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [update_mask][crate::model::UpdateProfileRequest::update_mask].
        pub fn set_or_clear_update_mask<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::FieldMask>
        {
            self.0.request.update_mask = v.map(|x| x.into());
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for UpdateProfile {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

}

pub mod export_service {
    use crate::Result;

    /// A builder for [ExportService][crate::client::ExportService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_profiler_v2::*;
    /// # use builder::export_service::ClientBuilder;
    /// # use client::ExportService;
    /// let builder : ClientBuilder = ExportService::builder();
    /// let client = builder
    ///     .with_endpoint("https://cloudprofiler.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::ExportService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = ExportService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::ExportService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::ExportService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where R: std::default::Default {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::ExportService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [ExportService::list_profiles][crate::client::ExportService::list_profiles] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_profiler_v2::builder;
    /// use builder::export_service::ListProfiles;
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
    /// fn prepare_request_builder() -> ListProfiles {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct ListProfiles(RequestBuilder<crate::model::ListProfilesRequest>);

    impl ListProfiles {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::ExportService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListProfilesRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListProfilesResponse> {
            (*self.0.stub).list_profiles(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Streams each page in the collection.
        pub fn by_page(self) -> impl gax::paginator::Paginator<crate::model::ListProfilesResponse, gax::error::Error> {
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
        pub fn by_item(self) -> impl gax::paginator::ItemPaginator<crate::model::ListProfilesResponse, gax::error::Error> {
            use gax::paginator::Paginator;
            self.by_page().items()
        }

        /// Sets the value of [parent][crate::model::ListProfilesRequest::parent].
        ///
        /// This is a **required** field for requests.
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListProfilesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListProfilesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListProfiles {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

}
