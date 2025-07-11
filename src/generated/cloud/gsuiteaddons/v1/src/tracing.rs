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

/// Implements a [GSuiteAddOns](super::stub::GSuiteAddOns) decorator for logging and tracing.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone, Debug)]
pub struct GSuiteAddOns<T>
where T: super::stub::GSuiteAddOns + std::fmt::Debug + Send + Sync {
    inner: T,
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Clone, Debug)]
pub struct GSuiteAddOns<T>
where T: super::stub::GSuiteAddOns + std::fmt::Debug {
    inner: T,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> GSuiteAddOns<T>
where T: super::stub::GSuiteAddOns + std::fmt::Debug + Send + Sync {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> GSuiteAddOns<T>
where T: super::stub::GSuiteAddOns + std::fmt::Debug {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T> super::stub::GSuiteAddOns for GSuiteAddOns<T>
where T: super::stub::GSuiteAddOns + std::fmt::Debug + Send + Sync {
    #[tracing::instrument(ret)]
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Authorization>> {
        self.inner.get_authorization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Deployment>> {
        self.inner.create_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Deployment>> {
        self.inner.replace_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Deployment>> {
        self.inner.get_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDeploymentsResponse>> {
        self.inner.list_deployments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.install_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.uninstall_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::InstallStatus>> {
        self.inner.get_install_status(req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl<T> super::stub::GSuiteAddOns for GSuiteAddOns<T>
where T: super::stub::GSuiteAddOns + std::fmt::Debug {
    #[tracing::instrument(ret)]
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Authorization>> {
        self.inner.get_authorization(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Deployment>> {
        self.inner.create_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Deployment>> {
        self.inner.replace_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Deployment>> {
        self.inner.get_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListDeploymentsResponse>> {
        self.inner.list_deployments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.delete_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.install_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        self.inner.uninstall_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::InstallStatus>> {
        self.inner.get_install_status(req, options).await
    }

}

