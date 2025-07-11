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

pub mod locations {
    use crate::Result;

    /// A builder for [Locations][crate::client::Locations].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_location::*;
    /// # use builder::locations::ClientBuilder;
    /// # use client::Locations;
    /// let builder : ClientBuilder = Locations::builder();
    /// let client = builder
    ///     .with_endpoint("https://cloud.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::Locations;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = Locations;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::Locations] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::Locations>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where R: std::default::Default {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::Locations>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [Locations::list_locations][crate::client::Locations::list_locations] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_location::builder;
    /// use builder::locations::ListLocations;
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
    /// fn prepare_request_builder() -> ListLocations {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct ListLocations(RequestBuilder<crate::model::ListLocationsRequest>);

    impl ListLocations {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::Locations>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListLocationsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListLocationsResponse> {
            (*self.0.stub).list_locations(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Streams each page in the collection.
        pub fn by_page(self) -> impl gax::paginator::Paginator<crate::model::ListLocationsResponse, gax::error::Error> {
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
        pub fn by_item(self) -> impl gax::paginator::ItemPaginator<crate::model::ListLocationsResponse, gax::error::Error> {
            use gax::paginator::Paginator;
            self.by_page().items()
        }

        /// Sets the value of [name][crate::model::ListLocationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListLocationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListLocationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListLocationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListLocations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [Locations::get_location][crate::client::Locations::get_location] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_location::builder;
    /// use builder::locations::GetLocation;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> GetLocation {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct GetLocation(RequestBuilder<crate::model::GetLocationRequest>);

    impl GetLocation {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::Locations>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetLocationRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Location> {
            (*self.0.stub).get_location(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::GetLocationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetLocation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

}
