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

/// Implements a [TranslationService](super::stub::TranslationService) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct TranslationService<T>
where T: super::stub::TranslationService + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct TranslationService<T>
where T: super::stub::TranslationService + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> TranslationService<T>
where T: super::stub::TranslationService + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> TranslationService<T>
where T: super::stub::TranslationService + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::TranslationService for TranslationService<T>
where T: super::stub::TranslationService + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn translate_text(
        &self,
        req: crate::model::TranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TranslateTextResponse>> {
        self.inner.translate_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn romanize_text(
        &self,
        req: crate::model::RomanizeTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::RomanizeTextResponse>> {
        self.inner.romanize_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn detect_language(
        &self,
        req: crate::model::DetectLanguageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DetectLanguageResponse>> {
        self.inner.detect_language(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_supported_languages(
        &self,
        req: crate::model::GetSupportedLanguagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SupportedLanguages>> {
        self.inner.get_supported_languages(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn translate_document(
        &self,
        req: crate::model::TranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TranslateDocumentResponse>> {
        self.inner.translate_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_translate_text(
        &self,
        req: crate::model::BatchTranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_translate_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_translate_document(
        &self,
        req: crate::model::BatchTranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_translate_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_glossary(
        &self,
        req: crate::model::CreateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_glossary(
        &self,
        req: crate::model::UpdateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_glossaries(
        &self,
        req: crate::model::ListGlossariesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListGlossariesResponse>> {
        self.inner.list_glossaries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_glossary(
        &self,
        req: crate::model::GetGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Glossary>> {
        self.inner.get_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_glossary(
        &self,
        req: crate::model::DeleteGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_glossary_entry(
        &self,
        req: crate::model::GetGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GlossaryEntry>> {
        self.inner.get_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_glossary_entries(
        &self,
        req: crate::model::ListGlossaryEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListGlossaryEntriesResponse>> {
        self.inner.list_glossary_entries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_glossary_entry(
        &self,
        req: crate::model::CreateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GlossaryEntry>> {
        self.inner.create_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_glossary_entry(
        &self,
        req: crate::model::UpdateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GlossaryEntry>> {
        self.inner.update_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_glossary_entry(
        &self,
        req: crate::model::DeleteGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_dataset(
        &self,
        req: crate::model::CreateDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dataset(
        &self,
        req: crate::model::GetDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Dataset>> {
        self.inner.get_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_datasets(
        &self,
        req: crate::model::ListDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatasetsResponse>> {
        self.inner.list_datasets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dataset(
        &self,
        req: crate::model::DeleteDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_adaptive_mt_dataset(
        &self,
        req: crate::model::CreateAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtDataset>> {
        self.inner.create_adaptive_mt_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_adaptive_mt_dataset(
        &self,
        req: crate::model::DeleteAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_adaptive_mt_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_adaptive_mt_dataset(
        &self,
        req: crate::model::GetAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtDataset>> {
        self.inner.get_adaptive_mt_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_adaptive_mt_datasets(
        &self,
        req: crate::model::ListAdaptiveMtDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAdaptiveMtDatasetsResponse>> {
        self.inner.list_adaptive_mt_datasets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn adaptive_mt_translate(
        &self,
        req: crate::model::AdaptiveMtTranslateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtTranslateResponse>> {
        self.inner.adaptive_mt_translate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_adaptive_mt_file(
        &self,
        req: crate::model::GetAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtFile>> {
        self.inner.get_adaptive_mt_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_adaptive_mt_file(
        &self,
        req: crate::model::DeleteAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_adaptive_mt_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_adaptive_mt_file(
        &self,
        req: crate::model::ImportAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ImportAdaptiveMtFileResponse>> {
        self.inner.import_adaptive_mt_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_adaptive_mt_files(
        &self,
        req: crate::model::ListAdaptiveMtFilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAdaptiveMtFilesResponse>> {
        self.inner.list_adaptive_mt_files(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_adaptive_mt_sentences(
        &self,
        req: crate::model::ListAdaptiveMtSentencesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAdaptiveMtSentencesResponse>> {
        self.inner.list_adaptive_mt_sentences(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_data(
        &self,
        req: crate::model::ImportDataRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.import_data(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn export_data(
        &self,
        req: crate::model::ExportDataRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.export_data(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_examples(
        &self,
        req: crate::model::ListExamplesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListExamplesResponse>> {
        self.inner.list_examples(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_model(
        &self,
        req: crate::model::CreateModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_model(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_models(
        &self,
        req: crate::model::ListModelsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListModelsResponse>> {
        self.inner.list_models(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_model(
        &self,
        req: crate::model::GetModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Model>> {
        self.inner.get_model(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_model(
        &self,
        req: crate::model::DeleteModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_model(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.wait_operation(req, options).await
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
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::TranslationService for TranslationService<T>
where T: super::stub::TranslationService + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn translate_text(
        &self,
        req: crate::model::TranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TranslateTextResponse>> {
        self.inner.translate_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn romanize_text(
        &self,
        req: crate::model::RomanizeTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::RomanizeTextResponse>> {
        self.inner.romanize_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn detect_language(
        &self,
        req: crate::model::DetectLanguageRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::DetectLanguageResponse>> {
        self.inner.detect_language(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_supported_languages(
        &self,
        req: crate::model::GetSupportedLanguagesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SupportedLanguages>> {
        self.inner.get_supported_languages(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn translate_document(
        &self,
        req: crate::model::TranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::TranslateDocumentResponse>> {
        self.inner.translate_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_translate_text(
        &self,
        req: crate::model::BatchTranslateTextRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_translate_text(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_translate_document(
        &self,
        req: crate::model::BatchTranslateDocumentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.batch_translate_document(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_glossary(
        &self,
        req: crate::model::CreateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_glossary(
        &self,
        req: crate::model::UpdateGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.update_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_glossaries(
        &self,
        req: crate::model::ListGlossariesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListGlossariesResponse>> {
        self.inner.list_glossaries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_glossary(
        &self,
        req: crate::model::GetGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Glossary>> {
        self.inner.get_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_glossary(
        &self,
        req: crate::model::DeleteGlossaryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_glossary(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_glossary_entry(
        &self,
        req: crate::model::GetGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GlossaryEntry>> {
        self.inner.get_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_glossary_entries(
        &self,
        req: crate::model::ListGlossaryEntriesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListGlossaryEntriesResponse>> {
        self.inner.list_glossary_entries(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_glossary_entry(
        &self,
        req: crate::model::CreateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GlossaryEntry>> {
        self.inner.create_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_glossary_entry(
        &self,
        req: crate::model::UpdateGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::GlossaryEntry>> {
        self.inner.update_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_glossary_entry(
        &self,
        req: crate::model::DeleteGlossaryEntryRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_glossary_entry(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_dataset(
        &self,
        req: crate::model::CreateDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dataset(
        &self,
        req: crate::model::GetDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Dataset>> {
        self.inner.get_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_datasets(
        &self,
        req: crate::model::ListDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDatasetsResponse>> {
        self.inner.list_datasets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dataset(
        &self,
        req: crate::model::DeleteDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_adaptive_mt_dataset(
        &self,
        req: crate::model::CreateAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtDataset>> {
        self.inner.create_adaptive_mt_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_adaptive_mt_dataset(
        &self,
        req: crate::model::DeleteAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_adaptive_mt_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_adaptive_mt_dataset(
        &self,
        req: crate::model::GetAdaptiveMtDatasetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtDataset>> {
        self.inner.get_adaptive_mt_dataset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_adaptive_mt_datasets(
        &self,
        req: crate::model::ListAdaptiveMtDatasetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAdaptiveMtDatasetsResponse>> {
        self.inner.list_adaptive_mt_datasets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn adaptive_mt_translate(
        &self,
        req: crate::model::AdaptiveMtTranslateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtTranslateResponse>> {
        self.inner.adaptive_mt_translate(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_adaptive_mt_file(
        &self,
        req: crate::model::GetAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::AdaptiveMtFile>> {
        self.inner.get_adaptive_mt_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_adaptive_mt_file(
        &self,
        req: crate::model::DeleteAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_adaptive_mt_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_adaptive_mt_file(
        &self,
        req: crate::model::ImportAdaptiveMtFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ImportAdaptiveMtFileResponse>> {
        self.inner.import_adaptive_mt_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_adaptive_mt_files(
        &self,
        req: crate::model::ListAdaptiveMtFilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAdaptiveMtFilesResponse>> {
        self.inner.list_adaptive_mt_files(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_adaptive_mt_sentences(
        &self,
        req: crate::model::ListAdaptiveMtSentencesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListAdaptiveMtSentencesResponse>> {
        self.inner.list_adaptive_mt_sentences(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn import_data(
        &self,
        req: crate::model::ImportDataRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.import_data(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn export_data(
        &self,
        req: crate::model::ExportDataRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.export_data(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_examples(
        &self,
        req: crate::model::ListExamplesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListExamplesResponse>> {
        self.inner.list_examples(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_model(
        &self,
        req: crate::model::CreateModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.create_model(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_models(
        &self,
        req: crate::model::ListModelsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListModelsResponse>> {
        self.inner.list_models(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_model(
        &self,
        req: crate::model::GetModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Model>> {
        self.inner.get_model(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_model(
        &self,
        req: crate::model::DeleteModelRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.delete_model(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::ListLocationsResponse>> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<location::model::Location>> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::ListOperationsResponse>> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.cancel_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn wait_operation(
        &self,
        req: longrunning::model::WaitOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<longrunning::model::Operation>> {
        self.inner.wait_operation(req, options).await
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

