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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::IamChecker].
///
/// Application developers may need to implement this trait to mock
/// `client::IamChecker`.  In other use-cases, application developers only
/// use `client::IamChecker` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait IamChecker: std::fmt::Debug + Send + Sync {

    /// Implements [super::client::IamChecker::troubleshoot_iam_policy].
    fn troubleshoot_iam_policy(
        &self,
        _req: crate::model::TroubleshootIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::TroubleshootIamPolicyResponse>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub trait IamChecker: std::fmt::Debug {

    /// Implements [super::client::IamChecker::troubleshoot_iam_policy].
    fn troubleshoot_iam_policy(
        &self,
        _req: crate::model::TroubleshootIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<crate::model::TroubleshootIamPolicyResponse>>> {
        gaxi::unimplemented::unimplemented_stub()
    }
}

