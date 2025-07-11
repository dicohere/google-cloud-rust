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

/// Implements [AssetService](super::stub::AssetService) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct AssetService {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for AssetService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("AssetService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl AssetService {
    pub async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::AssetService for AssetService {
    async fn export_assets(
        &self,
        req: crate::model::ExportAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:exportAssets",
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

    async fn list_assets(
        &self,
        req: crate::model::ListAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAssetsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/assets",
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
        let builder = req.read_time.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "readTime") });
        let builder = req.asset_types.iter().fold(builder, |builder, p| builder.query(&[("assetTypes", p)]));
        let builder = builder.query(&[("contentType", &req.content_type)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = req.relationship_types.iter().fold(builder, |builder, p| builder.query(&[("relationshipTypes", p)]));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn batch_get_assets_history(
        &self,
        req: crate::model::BatchGetAssetsHistoryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BatchGetAssetsHistoryResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:batchGetAssetsHistory",
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
        let builder = req.asset_names.iter().fold(builder, |builder, p| builder.query(&[("assetNames", p)]));
        let builder = builder.query(&[("contentType", &req.content_type)]);
        let builder = req.read_time_window.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "readTimeWindow") });
        let builder = req.relationship_types.iter().fold(builder, |builder, p| builder.query(&[("relationshipTypes", p)]));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn create_feed(
        &self,
        req: crate::model::CreateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Feed>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/feeds",
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

    async fn get_feed(
        &self,
        req: crate::model::GetFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Feed>> {
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

    async fn list_feeds(
        &self,
        req: crate::model::ListFeedsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListFeedsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/feeds",
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
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn update_feed(
        &self,
        req: crate::model::UpdateFeedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Feed>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.feed.as_ref().ok_or_else(|| gaxi::path_parameter::missing("feed"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("feed.name"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::PATCH, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
    }

    async fn delete_feed(
        &self,
        req: crate::model::DeleteFeedRequest,
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

    async fn search_all_resources(
        &self,
        req: crate::model::SearchAllResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchAllResourcesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:searchAllResources",
                    {
                        let arg = &req.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("query", &req.query)]);
        let builder = req.asset_types.iter().fold(builder, |builder, p| builder.query(&[("assetTypes", p)]));
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("orderBy", &req.order_by)]);
        let builder = req.read_mask.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "readMask") });
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn search_all_iam_policies(
        &self,
        req: crate::model::SearchAllIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SearchAllIamPoliciesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:searchAllIamPolicies",
                    {
                        let arg = &req.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("query", &req.query)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = req.asset_types.iter().fold(builder, |builder, p| builder.query(&[("assetTypes", p)]));
        let builder = builder.query(&[("orderBy", &req.order_by)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn analyze_iam_policy(
        &self,
        req: crate::model::AnalyzeIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeIamPolicyResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:analyzeIamPolicy",
                    {
                        let arg = &req.analysis_query.as_ref().ok_or_else(|| gaxi::path_parameter::missing("analysis_query"))?.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("analysis_query.scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.analysis_query.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "analysisQuery") });
        let builder = builder.query(&[("savedAnalysisQuery", &req.saved_analysis_query)]);
        let builder = req.execution_timeout.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "executionTimeout") });
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn analyze_iam_policy_longrunning(
        &self,
        req: crate::model::AnalyzeIamPolicyLongrunningRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:analyzeIamPolicyLongrunning",
                    {
                        let arg = &req.analysis_query.as_ref().ok_or_else(|| gaxi::path_parameter::missing("analysis_query"))?.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("analysis_query.scope"));
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

    async fn analyze_move(
        &self,
        req: crate::model::AnalyzeMoveRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeMoveResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:analyzeMove",
                    {
                        let arg = &req.resource;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("resource"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("destinationParent", &req.destination_parent)]);
        let builder = builder.query(&[("view", &req.view)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn query_assets(
        &self,
        req: crate::model::QueryAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::QueryAssetsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:queryAssets",
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

    async fn create_saved_query(
        &self,
        req: crate::model::CreateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SavedQuery>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/savedQueries",
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
        let builder = builder.query(&[("savedQueryId", &req.saved_query_id)]);
        self.inner.execute(
            builder,
            Some(req.saved_query),
            options,
        ).await
    }

    async fn get_saved_query(
        &self,
        req: crate::model::GetSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SavedQuery>> {
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

    async fn list_saved_queries(
        &self,
        req: crate::model::ListSavedQueriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListSavedQueriesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/savedQueries",
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
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn update_saved_query(
        &self,
        req: crate::model::UpdateSavedQueryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SavedQuery>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.saved_query.as_ref().ok_or_else(|| gaxi::path_parameter::missing("saved_query"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("saved_query.name"));
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
        self.inner.execute(
            builder,
            Some(req.saved_query),
            options,
        ).await
    }

    async fn delete_saved_query(
        &self,
        req: crate::model::DeleteSavedQueryRequest,
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

    async fn batch_get_effective_iam_policies(
        &self,
        req: crate::model::BatchGetEffectiveIamPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::BatchGetEffectiveIamPoliciesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/effectiveIamPolicies:batchGet",
                    {
                        let arg = &req.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.names.iter().fold(builder, |builder, p| builder.query(&[("names", p)]));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn analyze_org_policies(
        &self,
        req: crate::model::AnalyzeOrgPoliciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeOrgPoliciesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:analyzeOrgPolicies",
                    {
                        let arg = &req.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("constraint", &req.constraint)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = req.page_size.iter().fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn analyze_org_policy_governed_containers(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedContainersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeOrgPolicyGovernedContainersResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:analyzeOrgPolicyGovernedContainers",
                    {
                        let arg = &req.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("constraint", &req.constraint)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = req.page_size.iter().fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn analyze_org_policy_governed_assets(
        &self,
        req: crate::model::AnalyzeOrgPolicyGovernedAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeOrgPolicyGovernedAssetsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}:analyzeOrgPolicyGovernedAssets",
                    {
                        let arg = &req.scope;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("scope"));
                        }
                        arg
                    },
            );
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("constraint", &req.constraint)]);
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = req.page_size.iter().fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
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

