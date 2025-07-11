// Copyright 2024 Google LLC
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

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate std;
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate gaxi;
extern crate lazy_static;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate tracing;
extern crate wkt;

/// The request message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
///
/// [google.cloud.location.Locations.ListLocations]: crate::client::Locations::list_locations
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsRequest {

    /// The resource that owns the locations collection, if applicable.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub name: std::string::String,

    /// The standard list filter.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub filter: std::string::String,

    /// The standard list page size.
    #[serde(skip_serializing_if = "wkt::internal::is_default")]
    #[serde_as(as = "serde_with::DefaultOnNull<wkt::internal::I32>")]
    pub page_size: i32,

    /// The standard list page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub page_token: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl ListLocationsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::ListLocationsRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListLocationsRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListLocationsRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListLocationsRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListLocationsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.ListLocationsRequest"
    }
}

/// The response message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
///
/// [google.cloud.location.Locations.ListLocations]: crate::client::Locations::list_locations
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsResponse {

    /// A list of locations that matches the specified filter in the request.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<std::vec::Vec<_>>")]
    pub locations: std::vec::Vec<crate::model::Location>,

    /// The standard List next-page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub next_page_token: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl ListLocationsResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [locations][crate::model::ListLocationsResponse::locations].
    pub fn set_locations<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Location>
    {
        use std::iter::Iterator;
        self.locations = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [next_page_token][crate::model::ListLocationsResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }
}

impl wkt::message::Message for ListLocationsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.ListLocationsResponse"
    }
}

#[doc(hidden)]
impl gax::paginator::internal::PageableResponse for ListLocationsResponse {
    type PageItem = crate::model::Location;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.locations
    }

    fn next_page_token(&self) -> std::string::String {
        use std::clone::Clone;
        self.next_page_token.clone()
    }
}

/// The request message for [Locations.GetLocation][google.cloud.location.Locations.GetLocation].
///
/// [google.cloud.location.Locations.GetLocation]: crate::client::Locations::get_location
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetLocationRequest {

    /// Resource name for the location.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub name: std::string::String,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl GetLocationRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetLocationRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetLocationRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.GetLocationRequest"
    }
}

/// A resource that represents Google Cloud Platform location.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Location {

    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub name: std::string::String,

    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub location_id: std::string::String,

    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<_>")]
    pub display_name: std::string::String,

    /// Cross-service attributes for the location. For example
    ///
    /// ```norust
    /// {"cloud.googleapis.com/region": "us-east1"}
    /// ```
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde_as(as = "serde_with::DefaultOnNull<std::collections::HashMap<_, _>>")]
    pub labels: std::collections::HashMap<std::string::String,std::string::String>,

    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub metadata: std::option::Option<wkt::Any>,

    #[serde(flatten, skip_serializing_if = "serde_json::Map::is_empty")]
    _unknown_fields: serde_json::Map<std::string::String, serde_json::Value>,
}

impl Location {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Location::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [location_id][crate::model::Location::location_id].
    pub fn set_location_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.location_id = v.into();
        self
    }

    /// Sets the value of [display_name][crate::model::Location::display_name].
    pub fn set_display_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of [labels][crate::model::Location::labels].
    pub fn set_labels<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }

    /// Sets the value of [metadata][crate::model::Location::metadata].
    pub fn set_metadata<T>(mut self, v: T) -> Self
    where T: std::convert::Into<wkt::Any>
    {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    /// Sets or clears the value of [metadata][crate::model::Location::metadata].
    pub fn set_or_clear_metadata<T>(mut self, v: std::option::Option<T>) -> Self
    where T: std::convert::Into<wkt::Any>
    {
        self.metadata = v.map(|x| x.into());
        self
    }
}

impl wkt::message::Message for Location {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.location.Location"
    }
}
