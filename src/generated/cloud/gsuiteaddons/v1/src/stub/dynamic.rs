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

/// A dyn-compatible, crate-private version of [super::GSuiteAddOns].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
pub trait GSuiteAddOns: std::fmt::Debug + Send + Sync {
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Authorization>>;

    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>>;

    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>>;

    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>>;

    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDeploymentsResponse>>;

    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InstallStatus>>;

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
pub trait GSuiteAddOns: std::fmt::Debug {
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Authorization>>;

    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>>;

    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>>;

    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>>;

    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDeploymentsResponse>>;

    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>>;

    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InstallStatus>>;

}

/// All implementations of [super::GSuiteAddOns] also implement [GSuiteAddOns].
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[async_trait::async_trait]
impl<T: super::GSuiteAddOns> GSuiteAddOns for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Authorization>> {
        T::get_authorization(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>> {
        T::create_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>> {
        T::replace_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>> {
        T::get_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDeploymentsResponse>> {
        T::list_deployments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::install_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::uninstall_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InstallStatus>> {
        T::get_install_status(self, req, options).await
    }

}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[async_trait::async_trait(?Send)]
impl<T: super::GSuiteAddOns> GSuiteAddOns for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn get_authorization(
        &self,
        req: crate::model::GetAuthorizationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Authorization>> {
        T::get_authorization(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>> {
        T::create_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn replace_deployment(
        &self,
        req: crate::model::ReplaceDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>> {
        T::replace_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::Deployment>> {
        T::get_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::ListDeploymentsResponse>> {
        T::list_deployments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::delete_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn install_deployment(
        &self,
        req: crate::model::InstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::install_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn uninstall_deployment(
        &self,
        req: crate::model::UninstallDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<()>> {
        T::uninstall_deployment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_install_status(
        &self,
        req: crate::model::GetInstallStatusRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<gax::response::Response<crate::model::InstallStatus>> {
        T::get_install_status(self, req, options).await
    }

}
