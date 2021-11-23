// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Requester, Response};

use async_trait::async_trait;

/// A dummy HTTP client implementation that always errors.
#[derive(Debug)]
pub struct DummyClient;

#[async_trait]
impl Requester for DummyClient {
    async fn get(&self, _url: url::Url) -> Result<Response, ()> {
        Err(())
    }
}
