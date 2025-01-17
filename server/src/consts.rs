// Copyright 2023 The HoraeDB Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Common constants

/// Header of catalog name
pub const CATALOG_HEADER: &str = "x-horaedb-catalog";
/// Header of schema name
pub const SCHEMA_HEADER: &str = "x-horaedb-schema";
/// Header of tenant name
pub const TENANT_HEADER: &str = "x-horaedb-access-tenant";
/// Header of content encoding type
pub const CONTENT_ENCODING_HEADER: &str = "content-encoding";

pub const GZIP_ENCODING: &str = "gzip";
