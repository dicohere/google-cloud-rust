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

/// Implements [Iam](super::stub::Iam) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct Iam {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for Iam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Iam")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Iam {
    pub async fn new(config: gaxi::options::ClientConfig) -> gax::client_builder::Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::Iam for Iam {
    async fn list_service_accounts(
        &self,
        req: crate::model::ListServiceAccountsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListServiceAccountsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/serviceAccounts",
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
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn get_service_account(
        &self,
        req: crate::model::GetServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccount>> {
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

    async fn create_service_account(
        &self,
        req: crate::model::CreateServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccount>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/serviceAccounts",
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

    async fn update_service_account(
        &self,
        req: crate::model::ServiceAccount,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccount>> {
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
            .builder(reqwest::Method::PUT, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
    }

    async fn patch_service_account(
        &self,
        req: crate::model::PatchServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccount>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}",
                    {
                        let arg = &req.service_account.as_ref().ok_or_else(|| gaxi::path_parameter::missing("service_account"))?.name;
                        if arg.is_empty() {
                            return Err(gaxi::path_parameter::missing("service_account.name"));
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

    async fn delete_service_account(
        &self,
        req: crate::model::DeleteServiceAccountRequest,
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

    async fn undelete_service_account(
        &self,
        req: crate::model::UndeleteServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::UndeleteServiceAccountResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:undelete",
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

    async fn enable_service_account(
        &self,
        req: crate::model::EnableServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:enable",
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

    async fn disable_service_account(
        &self,
        req: crate::model::DisableServiceAccountRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:disable",
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

    async fn list_service_account_keys(
        &self,
        req: crate::model::ListServiceAccountKeysRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListServiceAccountKeysResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            format!("/v1/{}/keys",
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
        let builder = req.key_types.iter().fold(builder, |builder, p| builder.query(&[("keyTypes", p)]));
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn get_service_account_key(
        &self,
        req: crate::model::GetServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccountKey>> {
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
        let builder = builder.query(&[("publicKeyType", &req.public_key_type)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn create_service_account_key(
        &self,
        req: crate::model::CreateServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccountKey>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/keys",
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

    async fn upload_service_account_key(
        &self,
        req: crate::model::UploadServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ServiceAccountKey>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/keys:upload",
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

    async fn delete_service_account_key(
        &self,
        req: crate::model::DeleteServiceAccountKeyRequest,
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

    async fn disable_service_account_key(
        &self,
        req: crate::model::DisableServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:disable",
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

    async fn enable_service_account_key(
        &self,
        req: crate::model::EnableServiceAccountKeyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<()>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:enable",
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

    async fn sign_blob(
        &self,
        req: crate::model::SignBlobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SignBlobResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:signBlob",
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

    async fn sign_jwt(
        &self,
        req: crate::model::SignJwtRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::SignJwtResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:signJwt",
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

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:getIamPolicy",
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
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.options.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "options") });
        self.inner.execute(
            builder,
            Some(gaxi::http::NoBody),
            options,
        ).await
    }

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::Policy>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:setIamPolicy",
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
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
    }

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:testIamPermissions",
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
            .builder(reqwest::Method::POST, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req),
            options,
        ).await
    }

    async fn query_grantable_roles(
        &self,
        req: crate::model::QueryGrantableRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::QueryGrantableRolesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            "/v1/roles:queryGrantableRoles".to_string();
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

    async fn list_roles(
        &self,
        req: crate::model::ListRolesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::ListRolesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            true,
        );
        let path =
            "/v1/roles".to_string();
        let builder = self
            .inner
            .builder(reqwest::Method::GET, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = builder.query(&[("parent", &req.parent)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("view", &req.view)]);
        let builder = builder.query(&[("showDeleted", &req.show_deleted)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn get_role(
        &self,
        req: crate::model::GetRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Role>> {
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

    async fn create_role(
        &self,
        req: crate::model::CreateRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Role>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}/roles",
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

    async fn update_role(
        &self,
        req: crate::model::UpdateRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Role>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
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
            .builder(reqwest::Method::PATCH, path)
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        let builder = req.update_mask.as_ref().map(|p| serde_json::to_value(p).map_err(Error::ser) ).transpose()?.into_iter().fold(builder, |builder, v| { use gaxi::query_parameter::QueryParameter; v.add(builder, "updateMask") });
        self.inner.execute(
            builder,
            Some(req.role),
            options,
        ).await
    }

    async fn delete_role(
        &self,
        req: crate::model::DeleteRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Role>> {
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
        let builder = builder.query(&[("etag", &req.etag)]);
        self.inner.execute(
            builder,
            None::<gaxi::http::NoBody>,
            options,
        ).await
    }

    async fn undelete_role(
        &self,
        req: crate::model::UndeleteRoleRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::Role>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            format!("/v1/{}:undelete",
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

    async fn query_testable_permissions(
        &self,
        req: crate::model::QueryTestablePermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::QueryTestablePermissionsResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            "/v1/permissions:queryTestablePermissions".to_string();
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

    async fn query_auditable_services(
        &self,
        req: crate::model::QueryAuditableServicesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::QueryAuditableServicesResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            "/v1/iamPolicies:queryAuditableServices".to_string();
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

    async fn lint_policy(
        &self,
        req: crate::model::LintPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<gax::response::Response<crate::model::LintPolicyResponse>> {
        let options = gax::options::internal::set_default_idempotency(
            options,
            false,
        );
        let path =
            "/v1/iamPolicies:lintPolicy".to_string();
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

}

