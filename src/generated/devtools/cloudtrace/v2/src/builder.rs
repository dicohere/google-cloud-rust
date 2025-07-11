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

pub mod trace_service {
    use crate::Result;

    /// A builder for [TraceService][crate::client::TraceService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_trace_v2::*;
    /// # use builder::trace_service::ClientBuilder;
    /// # use client::TraceService;
    /// let builder : ClientBuilder = TraceService::builder();
    /// let client = builder
    ///     .with_endpoint("https://cloudtrace.googleapis.com")
    ///     .build().await?;
    /// # gax::client_builder::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::TraceService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = TraceService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [crate::client::TraceService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: std::sync::Arc<dyn super::super::stub::dynamic::TraceService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where R: std::default::Default {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::TraceService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [TraceService::batch_write_spans][crate::client::TraceService::batch_write_spans] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_trace_v2::builder;
    /// use builder::trace_service::BatchWriteSpans;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> BatchWriteSpans {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct BatchWriteSpans(RequestBuilder<crate::model::BatchWriteSpansRequest>);

    impl BatchWriteSpans {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::TraceService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::BatchWriteSpansRequest>>(mut self, v: V) -> Self {
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
            (*self.0.stub).batch_write_spans(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::BatchWriteSpansRequest::name].
        ///
        /// This is a **required** field for requests.
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [spans][crate::model::BatchWriteSpansRequest::spans].
        ///
        /// This is a **required** field for requests.
        pub fn set_spans<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::Span>
        {
            use std::iter::Iterator;
            self.0.request.spans = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for BatchWriteSpans {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [TraceService::create_span][crate::client::TraceService::create_span] calls.
    ///
    /// # Example
    /// ```no_run
    /// # use google_cloud_trace_v2::builder;
    /// use builder::trace_service::CreateSpan;
    /// # tokio_test::block_on(async {
    ///
    /// let builder = prepare_request_builder();
    /// let response = builder.send().await?;
    /// # gax::Result::<()>::Ok(()) });
    ///
    /// fn prepare_request_builder() -> CreateSpan {
    ///   # panic!();
    ///   // ... details omitted ...
    /// }
    /// ```
    #[derive(Clone, Debug)]
    pub struct CreateSpan(RequestBuilder<crate::model::Span>);

    impl CreateSpan {
        pub(crate) fn new(stub: std::sync::Arc<dyn super::super::stub::dynamic::TraceService>) -> Self {
            Self(
                RequestBuilder::new(stub)
            )
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::Span>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Span> {
            (*self.0.stub).create_span(self.0.request, self.0.options).await.map(gax::response::Response::into_body)
        }

        /// Sets the value of [name][crate::model::Span::name].
        ///
        /// This is a **required** field for requests.
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [span_id][crate::model::Span::span_id].
        ///
        /// This is a **required** field for requests.
        pub fn set_span_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.span_id = v.into();
            self
        }

        /// Sets the value of [parent_span_id][crate::model::Span::parent_span_id].
        pub fn set_parent_span_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent_span_id = v.into();
            self
        }

        /// Sets the value of [display_name][crate::model::Span::display_name].
        ///
        /// This is a **required** field for requests.
        pub fn set_display_name<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::TruncatableString>
        {
            self.0.request.display_name = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [display_name][crate::model::Span::display_name].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_display_name<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::TruncatableString>
        {
            self.0.request.display_name = v.map(|x| x.into());
            self
        }

        /// Sets the value of [start_time][crate::model::Span::start_time].
        ///
        /// This is a **required** field for requests.
        pub fn set_start_time<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::Timestamp>
        {
            self.0.request.start_time = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [start_time][crate::model::Span::start_time].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_start_time<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::Timestamp>
        {
            self.0.request.start_time = v.map(|x| x.into());
            self
        }

        /// Sets the value of [end_time][crate::model::Span::end_time].
        ///
        /// This is a **required** field for requests.
        pub fn set_end_time<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::Timestamp>
        {
            self.0.request.end_time = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [end_time][crate::model::Span::end_time].
        ///
        /// This is a **required** field for requests.
        pub fn set_or_clear_end_time<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::Timestamp>
        {
            self.0.request.end_time = v.map(|x| x.into());
            self
        }

        /// Sets the value of [attributes][crate::model::Span::attributes].
        pub fn set_attributes<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::span::Attributes>
        {
            self.0.request.attributes = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [attributes][crate::model::Span::attributes].
        pub fn set_or_clear_attributes<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::span::Attributes>
        {
            self.0.request.attributes = v.map(|x| x.into());
            self
        }

        /// Sets the value of [stack_trace][crate::model::Span::stack_trace].
        pub fn set_stack_trace<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::StackTrace>
        {
            self.0.request.stack_trace = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [stack_trace][crate::model::Span::stack_trace].
        pub fn set_or_clear_stack_trace<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::StackTrace>
        {
            self.0.request.stack_trace = v.map(|x| x.into());
            self
        }

        /// Sets the value of [time_events][crate::model::Span::time_events].
        pub fn set_time_events<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::span::TimeEvents>
        {
            self.0.request.time_events = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [time_events][crate::model::Span::time_events].
        pub fn set_or_clear_time_events<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::span::TimeEvents>
        {
            self.0.request.time_events = v.map(|x| x.into());
            self
        }

        /// Sets the value of [links][crate::model::Span::links].
        pub fn set_links<T>(mut self, v: T) -> Self
        where T: std::convert::Into<crate::model::span::Links>
        {
            self.0.request.links = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [links][crate::model::Span::links].
        pub fn set_or_clear_links<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<crate::model::span::Links>
        {
            self.0.request.links = v.map(|x| x.into());
            self
        }

        /// Sets the value of [status][crate::model::Span::status].
        pub fn set_status<T>(mut self, v: T) -> Self
        where T: std::convert::Into<rpc::model::Status>
        {
            self.0.request.status = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [status][crate::model::Span::status].
        pub fn set_or_clear_status<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<rpc::model::Status>
        {
            self.0.request.status = v.map(|x| x.into());
            self
        }

        /// Sets the value of [same_process_as_parent_span][crate::model::Span::same_process_as_parent_span].
        pub fn set_same_process_as_parent_span<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::BoolValue>
        {
            self.0.request.same_process_as_parent_span = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [same_process_as_parent_span][crate::model::Span::same_process_as_parent_span].
        pub fn set_or_clear_same_process_as_parent_span<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::BoolValue>
        {
            self.0.request.same_process_as_parent_span = v.map(|x| x.into());
            self
        }

        /// Sets the value of [child_span_count][crate::model::Span::child_span_count].
        pub fn set_child_span_count<T>(mut self, v: T) -> Self
        where T: std::convert::Into<wkt::Int32Value>
        {
            self.0.request.child_span_count = std::option::Option::Some(v.into());
            self
        }

        /// Sets or clears the value of [child_span_count][crate::model::Span::child_span_count].
        pub fn set_or_clear_child_span_count<T>(mut self, v: std::option::Option<T>) -> Self
        where T: std::convert::Into<wkt::Int32Value>
        {
            self.0.request.child_span_count = v.map(|x| x.into());
            self
        }

        /// Sets the value of [span_kind][crate::model::Span::span_kind].
        pub fn set_span_kind<T: Into<crate::model::span::SpanKind>>(mut self, v: T) -> Self {
            self.0.request.span_kind = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateSpan {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

}
