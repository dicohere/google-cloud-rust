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

use crate::Result;
#[allow(unused_imports)]
use gax::error::Error;

/// Implements [Lineage](super::stub::Lineage) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct Lineage {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for Lineage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Lineage")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Lineage {
    pub async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::Lineage for Lineage {
    async fn process_open_lineage_run_event(
        &self,
        req: crate::model::ProcessOpenLineageRunEventRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ProcessOpenLineageRunEventResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:processOpenLineageRunEvent",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner.execute(
            builder,
            Some(req.open_lineage),
            options,
        ).await
    }

    async fn create_process(
        &self,
        req: crate::model::CreateProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Process>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/processes",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner.execute(
            builder,
            Some(req.process),
            options,
        ).await
    }

    async fn update_process(
        &self,
        req: crate::model::UpdateProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Process>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.process.as_ref().ok_or_else(|| gaxi::path_parameter::missing("process"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("process.name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::PATCH, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.update_mask.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "updateMask") });
        let builder = builder.query(&[("allowMissing", &req.allow_missing)]);
        self.inner.execute(
            builder,
            Some(req.process),
            options,
        ).await
    }

    async fn get_process(
        &self,
        req: crate::model::GetProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Process>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn list_processes(
        &self,
        req: crate::model::ListProcessesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListProcessesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/processes",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn delete_process(
        &self,
        req: crate::model::DeleteProcessRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("allowMissing", &req.allow_missing)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn create_run(
        &self,
        req: crate::model::CreateRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Run>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/runs",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner.execute(
            builder,
            Some(req.run),
            options,
        ).await
    }

    async fn update_run(
        &self,
        req: crate::model::UpdateRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Run>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.run.as_ref().ok_or_else(|| gaxi::path_parameter::missing("run"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("run.name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::PATCH, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.update_mask.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "updateMask") });
        let builder = builder.query(&[("allowMissing", &req.allow_missing)]);
        self.inner.execute(
            builder,
            Some(req.run),
            options,
        ).await
    }

    async fn get_run(
        &self,
        req: crate::model::GetRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Run>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn list_runs(
        &self,
        req: crate::model::ListRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListRunsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/runs",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn delete_run(
        &self,
        req: crate::model::DeleteRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("allowMissing", &req.allow_missing)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn create_lineage_event(
        &self,
        req: crate::model::CreateLineageEventRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LineageEvent>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/lineageEvents",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("requestId", &req.request_id)]);
        self.inner.execute(
            builder,
            Some(req.lineage_event),
            options,
        ).await
    }

    async fn get_lineage_event(
        &self,
        req: crate::model::GetLineageEventRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LineageEvent>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn list_lineage_events(
        &self,
        req: crate::model::ListLineageEventsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListLineageEventsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/lineageEvents",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn delete_lineage_event(
        &self,
        req: crate::model::DeleteLineageEventRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("allowMissing", &req.allow_missing)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<wkt::Empty>| {
            let (parts, _) = r.into_parts();
            gax::response::Response::from_parts(parts, ()) 
        })
    }

    async fn search_links(
        &self,
        req: crate::model::SearchLinksRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchLinksResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:searchLinks",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
    }

    async fn batch_search_link_processes(
        &self,
        req: crate::model::BatchSearchLinkProcessesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BatchSearchLinkProcessesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:batchSearchLinkProcesses",
                    {
                        let arg = &req.parent;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("parent"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
    }

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/operations",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
        .map(|r: gax::response::Response<wkt::Empty>| {
            let (parts, _) = r.into_parts();
            gax::response::Response::from_parts(parts, ()) 
        })
    }

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:cancel",
                    {
                        let arg = &req.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
        .map(|r: gax::response::Response<wkt::Empty>| {
            let (parts, _) = r.into_parts();
            gax::response::Response::from_parts(parts, ()) 
        })
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

