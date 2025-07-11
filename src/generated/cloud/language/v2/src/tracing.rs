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

/// Implements a [LanguageService](super::stub::LanguageService) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct LanguageService<T>
where T: super::stub::LanguageService + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct LanguageService<T>
where T: super::stub::LanguageService + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> LanguageService<T>
where T: super::stub::LanguageService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> LanguageService<T>
where T: super::stub::LanguageService + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::LanguageService for LanguageService<T>
where T: super::stub::LanguageService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn analyze_sentiment(
        &self,
        req: crate::model::AnalyzeSentimentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeSentimentResponse>> {
        self.inner.analyze_sentiment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn analyze_entities(
        &self,
        req: crate::model::AnalyzeEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeEntitiesResponse>> {
        self.inner.analyze_entities(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn classify_text(
        &self,
        req: crate::model::ClassifyTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ClassifyTextResponse>> {
        self.inner.classify_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn moderate_text(
        &self,
        req: crate::model::ModerateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ModerateTextResponse>> {
        self.inner.moderate_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn annotate_text(
        &self,
        req: crate::model::AnnotateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnnotateTextResponse>> {
        self.inner.annotate_text(req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::LanguageService for LanguageService<T>
where T: super::stub::LanguageService + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn analyze_sentiment(
        &self,
        req: crate::model::AnalyzeSentimentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeSentimentResponse>> {
        self.inner.analyze_sentiment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn analyze_entities(
        &self,
        req: crate::model::AnalyzeEntitiesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnalyzeEntitiesResponse>> {
        self.inner.analyze_entities(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn classify_text(
        &self,
        req: crate::model::ClassifyTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ClassifyTextResponse>> {
        self.inner.classify_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn moderate_text(
        &self,
        req: crate::model::ModerateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ModerateTextResponse>> {
        self.inner.moderate_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn annotate_text(
        &self,
        req: crate::model::AnnotateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AnnotateTextResponse>> {
        self.inner.annotate_text(req, options).await
    }

}

