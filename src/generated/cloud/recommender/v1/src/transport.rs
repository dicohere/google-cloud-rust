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

/// Implements [Recommender](super::stub::Recommender) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct Recommender {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for Recommender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Recommender")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Recommender {
    pub async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::Recommender for Recommender {
    async fn list_insights(
        &self,
        req: crate::model::ListInsightsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListInsightsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/insights",
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
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn get_insight(
        &self,
        req: crate::model::GetInsightRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Insight>> {
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

    async fn mark_insight_accepted(
        &self,
        req: crate::model::MarkInsightAcceptedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Insight>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:markAccepted",
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
    }

    async fn list_recommendations(
        &self,
        req: crate::model::ListRecommendationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListRecommendationsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/recommendations",
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
        let builder = builder.query(&[("filter", &req.filter)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn get_recommendation(
        &self,
        req: crate::model::GetRecommendationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Recommendation>> {
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

    async fn mark_recommendation_dismissed(
        &self,
        req: crate::model::MarkRecommendationDismissedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Recommendation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:markDismissed",
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
    }

    async fn mark_recommendation_claimed(
        &self,
        req: crate::model::MarkRecommendationClaimedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Recommendation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:markClaimed",
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
    }

    async fn mark_recommendation_succeeded(
        &self,
        req: crate::model::MarkRecommendationSucceededRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Recommendation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:markSucceeded",
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
    }

    async fn mark_recommendation_failed(
        &self,
        req: crate::model::MarkRecommendationFailedRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Recommendation>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:markFailed",
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
    }

    async fn get_recommender_config(
        &self,
        req: crate::model::GetRecommenderConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::RecommenderConfig>> {
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

    async fn update_recommender_config(
        &self,
        req: crate::model::UpdateRecommenderConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::RecommenderConfig>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.recommender_config.as_ref().ok_or_else(|| gaxi::path_parameter::missing("recommender_config"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("recommender_config.name"));
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
        let builder = builder.query(&[("validateOnly", &req.validate_only)]);
        self.inner.execute(
            builder,
            Some(req.recommender_config),
            options,
        ).await
    }

    async fn get_insight_type_config(
        &self,
        req: crate::model::GetInsightTypeConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::InsightTypeConfig>> {
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

    async fn update_insight_type_config(
        &self,
        req: crate::model::UpdateInsightTypeConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::InsightTypeConfig>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.insight_type_config.as_ref().ok_or_else(|| gaxi::path_parameter::missing("insight_type_config"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("insight_type_config.name"));
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
        let builder = builder.query(&[("validateOnly", &req.validate_only)]);
        self.inner.execute(
            builder,
            Some(req.insight_type_config),
            options,
        ).await
    }

}

