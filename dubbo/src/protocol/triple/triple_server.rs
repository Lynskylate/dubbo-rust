/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::triple::server::builder::ServerBuilder;
use base::Url;

#[derive(Default, Clone)]
pub struct TripleServer {
    builder: ServerBuilder,
}

impl TripleServer {
    pub fn new() -> TripleServer {
        Self {
            builder: ServerBuilder::new(),
        }
    }

    pub async fn serve(mut self, url: Url) {
        self.builder = ServerBuilder::from(url);
        self.builder.build().serve().await.unwrap()
    }
}
