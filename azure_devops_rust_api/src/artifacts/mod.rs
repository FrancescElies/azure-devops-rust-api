// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: crate::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::Credential,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://feeds.dev.azure.com";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: crate::Credential) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Set per-call policies."]
    #[must_use]
    pub fn per_call_policies(
        mut self,
        policies: impl Into<Vec<std::sync::Arc<dyn azure_core::Policy>>>,
    ) -> Self {
        self.options = self.options.per_call_policies(policies);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &crate::Credential {
        &self.credential
    }
    #[allow(dead_code)]
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: &mut azure_core::Request,
    ) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: crate::Credential) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::Credential,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn artifact_details_client(&self) -> artifact_details::Client {
        artifact_details::Client(self.clone())
    }
    pub fn change_tracking_client(&self) -> change_tracking::Client {
        change_tracking::Client(self.clone())
    }
    pub fn feed_management_client(&self) -> feed_management::Client {
        feed_management::Client(self.clone())
    }
    pub fn feed_recycle_bin_client(&self) -> feed_recycle_bin::Client {
        feed_recycle_bin::Client(self.clone())
    }
    pub fn provenance_client(&self) -> provenance::Client {
        provenance::Client(self.clone())
    }
    pub fn recycle_bin_client(&self) -> recycle_bin::Client {
        recycle_bin::Client(self.clone())
    }
    pub fn retention_policies_client(&self) -> retention_policies::Client {
        retention_policies::Client(self.clone())
    }
    pub fn service_settings_client(&self) -> service_settings::Client {
        service_settings::Client(self.clone())
    }
}
pub mod service_settings {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all service-wide feed creation and administration permissions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        pub fn get_global_permissions(
            &self,
            organization: impl Into<String>,
        ) -> get_global_permissions::Builder {
            get_global_permissions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                include_ids: None,
            }
        }
        #[doc = "Set service-wide permissions that govern feed creation and administration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: New permissions for the organization."]
        pub fn set_global_permissions(
            &self,
            organization: impl Into<String>,
            body: Vec<models::GlobalPermission>,
        ) -> set_global_permissions::Builder {
            set_global_permissions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
            }
        }
    }
    pub mod get_global_permissions {
        use super::models;
        type Response = models::GlobalPermissionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) include_ids: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to add IdentityIds to the permission objects."]
            pub fn include_ids(mut self, include_ids: bool) -> Self {
                self.include_ids = Some(include_ids);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/packaging/globalpermissions",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_ids) = &this.include_ids {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeIds", &include_ids.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GlobalPermissionList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_global_permissions {
        use super::models;
        type Response = models::GlobalPermissionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::GlobalPermission>,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/_apis/packaging/globalpermissions",
                            this.client.endpoint(),
                            &this.organization
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GlobalPermissionList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod change_tracking {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Query to determine which feeds have changed since the last call, tracked through the provided continuation token. Only changes to a feed itself are returned and impact the continuation token, not additions or alterations to packages within the feeds.\n\nIf the project parameter is present, gets all feed changes in the given project.\nIf omitted, gets all feed changes in the organization."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feed_changes(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feed_changes::Builder {
            get_feed_changes::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                include_deleted: None,
                continuation_token: None,
                batch_size: None,
            }
        }
        #[doc = "Query a feed to determine its current state.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feed_change(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feed_change::Builder {
            get_feed_change::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a batch of package changes made to a feed.  The changes returned are 'most recent change' so if an Add is followed by an Update before you begin enumerating, you'll only see one change in the batch.  While consuming batches using the continuation token, you may see changes to the same package version multiple times if they are happening as you enumerate.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_changes(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_changes::Builder {
            get_package_changes::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
                continuation_token: None,
                batch_size: None,
            }
        }
    }
    pub mod get_feed_changes {
        use super::models;
        type Response = models::FeedChangesResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) continuation_token: Option<i64>,
            pub(crate) batch_size: Option<i32>,
        }
        impl Builder {
            #[doc = "If true, get changes for all feeds including deleted feeds. The default value is false."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "A continuation token which acts as a bookmark to a previously retrieved change. This token allows the user to continue retrieving changes in batches, picking up where the previous batch left off. If specified, all the changes that occur strictly after the token will be returned. If not specified or 0, iteration will start with the first change."]
            pub fn continuation_token(mut self, continuation_token: i64) -> Self {
                self.continuation_token = Some(continuation_token);
                self
            }
            #[doc = "Number of package changes to fetch. The default value is 1000. The maximum value is 2000."]
            pub fn batch_size(mut self, batch_size: i32) -> Self {
                self.batch_size = Some(batch_size);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feedchanges",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", &continuation_token.to_string());
                        }
                        if let Some(batch_size) = &this.batch_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("batchSize", &batch_size.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedChangesResponse =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_feed_change {
        use super::models;
        type Response = models::FeedChange;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feedchanges/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedChange =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_changes {
        use super::models;
        type Response = models::PackageChangesResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
            pub(crate) continuation_token: Option<i64>,
            pub(crate) batch_size: Option<i32>,
        }
        impl Builder {
            #[doc = "A continuation token which acts as a bookmark to a previously retrieved change. This token allows the user to continue retrieving changes in batches, picking up where the previous batch left off. If specified, all the changes that occur strictly after the token will be returned. If not specified or 0, iteration will start with the first change."]
            pub fn continuation_token(mut self, continuation_token: i64) -> Self {
                self.continuation_token = Some(continuation_token);
                self
            }
            #[doc = "Number of package changes to fetch. The default value is 1000. The maximum value is 2000."]
            pub fn batch_size(mut self, batch_size: i32) -> Self {
                self.batch_size = Some(batch_size);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/packagechanges",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("continuationToken", &continuation_token.to_string());
                        }
                        if let Some(batch_size) = &this.batch_size {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("batchSize", &batch_size.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageChangesResponse =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod feed_recycle_bin {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Query for feeds within the recycle bin.\n\nIf the project parameter is present, gets all feeds in recycle bin in the given project.\nIf omitted, gets all feeds in recycle bin in the organization."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn list(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn restore_deleted_feed(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::JsonPatchDocument>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> restore_deleted_feed::Builder {
            restore_deleted_feed::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn permanent_delete_feed(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> permanent_delete_feed::Builder {
            permanent_delete_feed::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::FeedList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feedrecyclebin",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod restore_deleted_feed {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::JsonPatchDocument,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feedrecyclebin/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json-patch+json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod permanent_delete_feed {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feedrecyclebin/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod feed_management {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all feeds in an account where you have the provided role access.\n\nIf the project parameter is present, gets all feeds in the given project.\nIf omitted, gets all feeds in the organization."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feeds(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feeds::Builder {
            get_feeds::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                feed_role: None,
                include_deleted_upstreams: None,
                include_urls: None,
            }
        }
        #[doc = "Create a feed, a container for various package types.\n\nFeeds can be created in a project if the project parameter is included in the request url.\nIf the project parameter is omitted, the feed will not be associated with a project and will be created at the organization level."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A JSON object containing both required and optional attributes for the feed. Name is the only required value."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create_feed(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::Feed>,
            project: impl Into<String>,
        ) -> create_feed::Builder {
            create_feed::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the settings for a specific feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feed(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feed::Builder {
            get_feed::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
                include_deleted_upstreams: None,
            }
        }
        #[doc = "Change the attributes of a feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: A JSON object containing the feed settings to be updated."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_feed(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::FeedUpdate>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_feed::Builder {
            update_feed::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Remove a feed and all its packages. The feed moves to the recycle bin and is reversible.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_feed(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_feed::Builder {
            delete_feed::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get the permissions for a feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feed_permissions(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feed_permissions::Builder {
            get_feed_permissions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
                include_ids: None,
                exclude_inherited_permissions: None,
                identity_descriptor: None,
                include_deleted_feeds: None,
            }
        }
        #[doc = "Update the permissions on a feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Permissions to set."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_feed_permissions(
            &self,
            organization: impl Into<String>,
            body: Vec<models::FeedPermission>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> set_feed_permissions::Builder {
            set_feed_permissions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get all views for a feed.\n\nThe project parameter must be supplied if the feed was created in a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feed_views(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feed_views::Builder {
            get_feed_views::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Create a new view on the referenced feed.\n\nThe project parameter must be supplied if the feed was created in a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: View to be created."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create_feed_view(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::FeedView>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> create_feed_view::Builder {
            create_feed_view::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a view by Id.\n\nThe project parameter must be supplied if the feed was created in a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `view_id`: Name or Id of the view."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_feed_view(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            view_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_feed_view::Builder {
            get_feed_view::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                view_id: view_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Update a view.\n\nThe project parameter must be supplied if the feed was created in a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: New settings to apply to the specified view."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `view_id`: Name or Id of the view."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn update_feed_view(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::FeedView>,
            feed_id: impl Into<String>,
            view_id: impl Into<String>,
            project: impl Into<String>,
        ) -> update_feed_view::Builder {
            update_feed_view::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                view_id: view_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete a feed view.\n\nThe project parameter must be supplied if the feed was created in a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `view_id`: Name or Id of the view."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_feed_view(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            view_id: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_feed_view::Builder {
            delete_feed_view::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                view_id: view_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_feeds {
        use super::models;
        type Response = models::FeedList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) feed_role: Option<String>,
            pub(crate) include_deleted_upstreams: Option<bool>,
            pub(crate) include_urls: Option<bool>,
        }
        impl Builder {
            #[doc = "Filter by this role, either Administrator(4), Contributor(3), or Reader(2) level permissions."]
            pub fn feed_role(mut self, feed_role: impl Into<String>) -> Self {
                self.feed_role = Some(feed_role.into());
                self
            }
            #[doc = "Include upstreams that have been deleted in the response."]
            pub fn include_deleted_upstreams(mut self, include_deleted_upstreams: bool) -> Self {
                self.include_deleted_upstreams = Some(include_deleted_upstreams);
                self
            }
            #[doc = "Resolve names if true"]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(feed_role) = &this.feed_role {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("feedRole", feed_role);
                        }
                        if let Some(include_deleted_upstreams) = &this.include_deleted_upstreams {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDeletedUpstreams",
                                &include_deleted_upstreams.to_string(),
                            );
                        }
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod create_feed {
        use super::models;
        type Response = models::Feed;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::Feed,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Feed = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_feed {
        use super::models;
        type Response = models::Feed;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
            pub(crate) include_deleted_upstreams: Option<bool>,
        }
        impl Builder {
            #[doc = "Include upstreams that have been deleted in the response."]
            pub fn include_deleted_upstreams(mut self, include_deleted_upstreams: bool) -> Self {
                self.include_deleted_upstreams = Some(include_deleted_upstreams);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_deleted_upstreams) = &this.include_deleted_upstreams {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDeletedUpstreams",
                                &include_deleted_upstreams.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Feed = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_feed {
        use super::models;
        type Response = models::Feed;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::FeedUpdate,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Feed = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_feed {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/feeds/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_feed_permissions {
        use super::models;
        type Response = models::FeedPermissionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
            pub(crate) include_ids: Option<bool>,
            pub(crate) exclude_inherited_permissions: Option<bool>,
            pub(crate) identity_descriptor: Option<String>,
            pub(crate) include_deleted_feeds: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to include user Ids in the response.  Default is false."]
            pub fn include_ids(mut self, include_ids: bool) -> Self {
                self.include_ids = Some(include_ids);
                self
            }
            #[doc = "Set to true to only return explicitly set permissions on the feed.  Default is false."]
            pub fn exclude_inherited_permissions(
                mut self,
                exclude_inherited_permissions: bool,
            ) -> Self {
                self.exclude_inherited_permissions = Some(exclude_inherited_permissions);
                self
            }
            #[doc = "Filter permissions to the provided identity."]
            pub fn identity_descriptor(mut self, identity_descriptor: impl Into<String>) -> Self {
                self.identity_descriptor = Some(identity_descriptor.into());
                self
            }
            #[doc = "If includeDeletedFeeds is true, then feedId must be specified by name and not by Guid."]
            pub fn include_deleted_feeds(mut self, include_deleted_feeds: bool) -> Self {
                self.include_deleted_feeds = Some(include_deleted_feeds);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/permissions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_ids) = &this.include_ids {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeIds", &include_ids.to_string());
                        }
                        if let Some(exclude_inherited_permissions) =
                            &this.exclude_inherited_permissions
                        {
                            req.url_mut().query_pairs_mut().append_pair(
                                "excludeInheritedPermissions",
                                &exclude_inherited_permissions.to_string(),
                            );
                        }
                        if let Some(identity_descriptor) = &this.identity_descriptor {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("identityDescriptor", identity_descriptor);
                        }
                        if let Some(include_deleted_feeds) = &this.include_deleted_feeds {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDeletedFeeds",
                                &include_deleted_feeds.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedPermissionList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_feed_permissions {
        use super::models;
        type Response = models::FeedPermissionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::FeedPermission>,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/permissions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedPermissionList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_feed_views {
        use super::models;
        type Response = models::FeedViewList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/views",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedViewList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod create_feed_view {
        use super::models;
        type Response = models::FeedView;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::FeedView,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/views",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedView =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_feed_view {
        use super::models;
        type Response = models::FeedView;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) view_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/views/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.view_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedView =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod update_feed_view {
        use super::models;
        type Response = models::FeedView;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::FeedView,
            pub(crate) feed_id: String,
            pub(crate) view_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/views/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.view_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedView =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_feed_view {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) view_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/views/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.view_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod artifact_details {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query_package_metrics(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageMetricsQuery>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> query_package_metrics::Builder {
            query_package_metrics::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get details about all of the packages in the feed. Use the various filters to include or exclude information from the result set.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_packages(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_packages::Builder {
            get_packages::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
                protocol_type: None,
                package_name_query: None,
                normalized_package_name: None,
                include_urls: None,
                include_all_versions: None,
                is_listed: None,
                get_top_package_versions: None,
                is_release: None,
                include_description: None,
                top: None,
                skip: None,
                include_deleted: None,
                is_cached: None,
                direct_upstream_id: None,
            }
        }
        #[doc = "Get details about a specific package.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: The package Id (GUID Id, not the package name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package::Builder {
            get_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                project: project.into(),
                include_all_versions: None,
                include_urls: None,
                is_listed: None,
                is_release: None,
                include_deleted: None,
                include_description: None,
            }
        }
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn query_package_version_metrics(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageVersionMetricsQuery>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            project: impl Into<String>,
        ) -> query_package_version_metrics::Builder {
            query_package_version_metrics::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get a list of package versions, optionally filtering by state.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: Id of the package (GUID Id, not name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_versions(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_versions::Builder {
            get_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                project: project.into(),
                include_urls: None,
                is_listed: None,
                is_deleted: None,
            }
        }
        #[doc = "Get details about a specific package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: Id of the package (GUID Id, not name)."]
        #[doc = "* `package_version_id`: Id of the package version (GUID Id, not name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            package_version_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version::Builder {
            get_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                package_version_id: package_version_id.into(),
                project: project.into(),
                include_urls: None,
                is_listed: None,
                is_deleted: None,
            }
        }
        #[doc = "Gets provenance for a package version.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: Id of the package (GUID Id, not name)."]
        #[doc = "* `package_version_id`: Id of the package version (GUID Id, not name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_package_version_provenance(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            package_version_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_package_version_provenance::Builder {
            get_package_version_provenance::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                package_version_id: package_version_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Generate a SVG badge for the latest version of a package.  The generated SVG is typically used as the image in an HTML link which takes users to the feed containing the package to accelerate discovery and consumption.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: Id of the package (GUID Id, not name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_badge(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_badge::Builder {
            get_badge::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod query_package_metrics {
        use super::models;
        type Response = models::PackageMetricsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageMetricsQuery,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/packagemetricsbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageMetricsList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_packages {
        use super::models;
        type Response = models::PackageList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
            pub(crate) protocol_type: Option<String>,
            pub(crate) package_name_query: Option<String>,
            pub(crate) normalized_package_name: Option<String>,
            pub(crate) include_urls: Option<bool>,
            pub(crate) include_all_versions: Option<bool>,
            pub(crate) is_listed: Option<bool>,
            pub(crate) get_top_package_versions: Option<bool>,
            pub(crate) is_release: Option<bool>,
            pub(crate) include_description: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) is_cached: Option<bool>,
            pub(crate) direct_upstream_id: Option<String>,
        }
        impl Builder {
            #[doc = "One of the supported artifact package types."]
            pub fn protocol_type(mut self, protocol_type: impl Into<String>) -> Self {
                self.protocol_type = Some(protocol_type.into());
                self
            }
            #[doc = "Filter to packages that contain the provided string. Characters in the string must conform to the package name constraints."]
            pub fn package_name_query(mut self, package_name_query: impl Into<String>) -> Self {
                self.package_name_query = Some(package_name_query.into());
                self
            }
            #[doc = "[Obsolete] Used for legacy scenarios and may be removed in future versions."]
            pub fn normalized_package_name(
                mut self,
                normalized_package_name: impl Into<String>,
            ) -> Self {
                self.normalized_package_name = Some(normalized_package_name.into());
                self
            }
            #[doc = "Set to true to return REST Urls with the response. Default is True."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            #[doc = "Set to true to return all versions of the package in the response. Default is false (latest version only)."]
            pub fn include_all_versions(mut self, include_all_versions: bool) -> Self {
                self.include_all_versions = Some(include_all_versions);
                self
            }
            #[doc = "Only applicable for NuGet packages, setting it for other package types will result in a 404. If false, delisted package versions will be returned. Use this to filter the response when include_all_versions is set to true. Default is unset (do not return delisted packages)."]
            pub fn is_listed(mut self, is_listed: bool) -> Self {
                self.is_listed = Some(is_listed);
                self
            }
            #[doc = "Changes the behavior of $top and $skip to return all versions of each package up to $top. Must be used in conjunction with include_all_versions=true"]
            pub fn get_top_package_versions(mut self, get_top_package_versions: bool) -> Self {
                self.get_top_package_versions = Some(get_top_package_versions);
                self
            }
            #[doc = "Only applicable for Nuget packages. Use this to filter the response when include_all_versions is set to true. Default is True (only return packages without prerelease versioning)."]
            pub fn is_release(mut self, is_release: bool) -> Self {
                self.is_release = Some(is_release);
                self
            }
            #[doc = "Return the description for every version of each package in the response. Default is False."]
            pub fn include_description(mut self, include_description: bool) -> Self {
                self.include_description = Some(include_description);
                self
            }
            #[doc = "Get the top N packages (or package versions where get_top_package_versions=true)"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Skip the first N packages (or package versions where get_top_package_versions=true)"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "Return deleted or unpublished versions of packages in the response. Default is False."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "[Obsolete] Used for legacy scenarios and may be removed in future versions."]
            pub fn is_cached(mut self, is_cached: bool) -> Self {
                self.is_cached = Some(is_cached);
                self
            }
            #[doc = "Filter results to return packages from a specific upstream."]
            pub fn direct_upstream_id(mut self, direct_upstream_id: impl Into<String>) -> Self {
                self.direct_upstream_id = Some(direct_upstream_id.into());
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/packages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(protocol_type) = &this.protocol_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("protocolType", protocol_type);
                        }
                        if let Some(package_name_query) = &this.package_name_query {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("packageNameQuery", package_name_query);
                        }
                        if let Some(normalized_package_name) = &this.normalized_package_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("normalizedPackageName", normalized_package_name);
                        }
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        if let Some(include_all_versions) = &this.include_all_versions {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeAllVersions",
                                &include_all_versions.to_string(),
                            );
                        }
                        if let Some(is_listed) = &this.is_listed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isListed", &is_listed.to_string());
                        }
                        if let Some(get_top_package_versions) = &this.get_top_package_versions {
                            req.url_mut().query_pairs_mut().append_pair(
                                "getTopPackageVersions",
                                &get_top_package_versions.to_string(),
                            );
                        }
                        if let Some(is_release) = &this.is_release {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isRelease", &is_release.to_string());
                        }
                        if let Some(include_description) = &this.include_description {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDescription",
                                &include_description.to_string(),
                            );
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(is_cached) = &this.is_cached {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isCached", &is_cached.to_string());
                        }
                        if let Some(direct_upstream_id) = &this.direct_upstream_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("directUpstreamId", direct_upstream_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) project: String,
            pub(crate) include_all_versions: Option<bool>,
            pub(crate) include_urls: Option<bool>,
            pub(crate) is_listed: Option<bool>,
            pub(crate) is_release: Option<bool>,
            pub(crate) include_deleted: Option<bool>,
            pub(crate) include_description: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to return all versions of the package in the response. Default is false (latest version only)."]
            pub fn include_all_versions(mut self, include_all_versions: bool) -> Self {
                self.include_all_versions = Some(include_all_versions);
                self
            }
            #[doc = "Set to true to return REST Urls with the response. Default is True."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            #[doc = "Only applicable for NuGet packages, setting it for other package types will result in a 404. If false, delisted package versions will be returned. Use this to filter the response when include_all_versions is set to true. Default is unset (do not return delisted packages)."]
            pub fn is_listed(mut self, is_listed: bool) -> Self {
                self.is_listed = Some(is_listed);
                self
            }
            #[doc = "Only applicable for Nuget packages. Use this to filter the response when include_all_versions is set to true.  Default is True (only return packages without prerelease versioning)."]
            pub fn is_release(mut self, is_release: bool) -> Self {
                self.is_release = Some(is_release);
                self
            }
            #[doc = "Return deleted or unpublished versions of packages in the response. Default is False."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            #[doc = "Return the description for every version of each package in the response. Default is False."]
            pub fn include_description(mut self, include_description: bool) -> Self {
                self.include_description = Some(include_description);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/packages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_all_versions) = &this.include_all_versions {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeAllVersions",
                                &include_all_versions.to_string(),
                            );
                        }
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        if let Some(is_listed) = &this.is_listed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isListed", &is_listed.to_string());
                        }
                        if let Some(is_release) = &this.is_release {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isRelease", &is_release.to_string());
                        }
                        if let Some(include_deleted) = &this.include_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeDeleted", &include_deleted.to_string());
                        }
                        if let Some(include_description) = &this.include_description {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeDescription",
                                &include_description.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod query_package_version_metrics {
        use super::models;
        type Response = models::PackageVersionMetricsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageVersionMetricsQuery,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/Packages/{}/versionmetricsbatch",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageVersionMetricsList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_versions {
        use super::models;
        type Response = models::PackageVersionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) project: String,
            pub(crate) include_urls: Option<bool>,
            pub(crate) is_listed: Option<bool>,
            pub(crate) is_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to include urls for each version. Default is true."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            #[doc = "Only applicable for NuGet packages. If false, delisted package versions will be returned."]
            pub fn is_listed(mut self, is_listed: bool) -> Self {
                self.is_listed = Some(is_listed);
                self
            }
            #[doc = "If set specifies whether to return only deleted or non-deleted versions of packages in the response. Default is unset (return all versions)."]
            pub fn is_deleted(mut self, is_deleted: bool) -> Self {
                self.is_deleted = Some(is_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/Packages/{}/versions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        if let Some(is_listed) = &this.is_listed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isListed", &is_listed.to_string());
                        }
                        if let Some(is_deleted) = &this.is_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isDeleted", &is_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageVersionList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version {
        use super::models;
        type Response = models::PackageVersion;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) package_version_id: String,
            pub(crate) project: String,
            pub(crate) include_urls: Option<bool>,
            pub(crate) is_listed: Option<bool>,
            pub(crate) is_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to include urls for each version. Default is true."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            #[doc = "Only applicable for NuGet packages. If false, delisted package versions will be returned."]
            pub fn is_listed(mut self, is_listed: bool) -> Self {
                self.is_listed = Some(is_listed);
                self
            }
            #[doc = "This does not have any effect on the requested package version, for other versions returned specifies whether to return only deleted or non-deleted versions of packages in the response. Default is unset (return all versions)."]
            pub fn is_deleted(mut self, is_deleted: bool) -> Self {
                self.is_deleted = Some(is_deleted);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/Packages/{}/versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id,
                            &this.package_version_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        if let Some(is_listed) = &this.is_listed {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isListed", &is_listed.to_string());
                        }
                        if let Some(is_deleted) = &this.is_deleted {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("isDeleted", &is_deleted.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageVersion =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_package_version_provenance {
        use super::models;
        type Response = models::PackageVersionProvenance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) package_version_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/Packages/{}/Versions/{}/provenance",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id,
                            &this.package_version_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageVersionProvenance =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_badge {
        use super::models;
        type Response = String;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/public/packaging/Feeds/{}/Packages/{}/badge",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: String = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod recycle_bin {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Query for packages within the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_recycle_bin_packages(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_recycle_bin_packages::Builder {
            get_recycle_bin_packages::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
                protocol_type: None,
                package_name_query: None,
                include_urls: None,
                top: None,
                skip: None,
                include_all_versions: None,
            }
        }
        #[doc = "Queues a job to remove all package versions from a feed's recycle bin"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn empty_recycle_bin(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> empty_recycle_bin::Builder {
            empty_recycle_bin::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Get information about a package and all its versions within the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: The package Id (GUID Id, not the package name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_recycle_bin_package(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_recycle_bin_package::Builder {
            get_recycle_bin_package::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                project: project.into(),
                include_urls: None,
            }
        }
        #[doc = "Get a list of package versions within the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: The package Id (GUID Id, not the package name)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_recycle_bin_package_versions(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_recycle_bin_package_versions::Builder {
            get_recycle_bin_package_versions::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                project: project.into(),
                include_urls: None,
            }
        }
        #[doc = "Get information about a package version within the recycle bin.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or Id of the feed."]
        #[doc = "* `package_id`: The package Id (GUID Id, not the package name)."]
        #[doc = "* `package_version_id`: The package version Id 9guid Id, not the version string)."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_recycle_bin_package_version(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            package_id: impl Into<String>,
            package_version_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_recycle_bin_package_version::Builder {
            get_recycle_bin_package_version::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                package_id: package_id.into(),
                package_version_id: package_version_id.into(),
                project: project.into(),
                include_urls: None,
            }
        }
    }
    pub mod get_recycle_bin_packages {
        use super::models;
        type Response = models::PackageList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
            pub(crate) protocol_type: Option<String>,
            pub(crate) package_name_query: Option<String>,
            pub(crate) include_urls: Option<bool>,
            pub(crate) top: Option<i32>,
            pub(crate) skip: Option<i32>,
            pub(crate) include_all_versions: Option<bool>,
        }
        impl Builder {
            #[doc = "Type of package (e.g. NuGet, npm, ...)."]
            pub fn protocol_type(mut self, protocol_type: impl Into<String>) -> Self {
                self.protocol_type = Some(protocol_type.into());
                self
            }
            #[doc = "Filter to packages matching this name."]
            pub fn package_name_query(mut self, package_name_query: impl Into<String>) -> Self {
                self.package_name_query = Some(package_name_query.into());
                self
            }
            #[doc = "Set to true to return REST Urls with the response.  Default is True."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            #[doc = "Get the top N packages."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Skip the first N packages."]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "Set to true to return all versions of the package in the response.  Default is false (latest version only)."]
            pub fn include_all_versions(mut self, include_all_versions: bool) -> Self {
                self.include_all_versions = Some(include_all_versions);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/RecycleBin/Packages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(protocol_type) = &this.protocol_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("protocolType", protocol_type);
                        }
                        if let Some(package_name_query) = &this.package_name_query {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("packageNameQuery", package_name_query);
                        }
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$top", &top.to_string());
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("$skip", &skip.to_string());
                        }
                        if let Some(include_all_versions) = &this.include_all_versions {
                            req.url_mut().query_pairs_mut().append_pair(
                                "includeAllVersions",
                                &include_all_versions.to_string(),
                            );
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PackageList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod empty_recycle_bin {
        use super::models;
        type Response = models::OperationReference;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/RecycleBin/Packages",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationReference =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_recycle_bin_package {
        use super::models;
        type Response = models::Package;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) project: String,
            pub(crate) include_urls: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to return REST Urls with the response.  Default is True."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/RecycleBin/Packages/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Package = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_recycle_bin_package_versions {
        use super::models;
        type Response = models::RecycleBinPackageVersionList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) project: String,
            pub(crate) include_urls: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to return REST Urls with the response.  Default is True."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/RecycleBin/Packages/{}/Versions",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecycleBinPackageVersionList =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod get_recycle_bin_package_version {
        use super::models;
        type Response = models::RecycleBinPackageVersion;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) package_id: String,
            pub(crate) package_version_id: String,
            pub(crate) project: String,
            pub(crate) include_urls: Option<bool>,
        }
        impl Builder {
            #[doc = "Set to true to return REST Urls with the response.  Default is True."]
            pub fn include_urls(mut self, include_urls: bool) -> Self {
                self.include_urls = Some(include_urls);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/RecycleBin/Packages/{}/Versions/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id,
                            &this.package_id,
                            &this.package_version_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        if let Some(include_urls) = &this.include_urls {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeUrls", &include_urls.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RecycleBinPackageVersion =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod retention_policies {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the retention policy for a feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn get_retention_policy(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> get_retention_policy::Builder {
            get_retention_policy::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Set the retention policy for a feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: Feed retention policy."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn set_retention_policy(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::FeedRetentionPolicy>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> set_retention_policy::Builder {
            set_retention_policy::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
        #[doc = "Delete the retention policy for a feed.\n\nThe project parameter must be supplied if the feed was created in a project.\nIf the feed is not associated with any project, omit the project parameter from the request."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `feed_id`: Name or ID of the feed."]
        #[doc = "* `project`: Project ID or project name"]
        pub fn delete_retention_policy(
            &self,
            organization: impl Into<String>,
            feed_id: impl Into<String>,
            project: impl Into<String>,
        ) -> delete_retention_policy::Builder {
            delete_retention_policy::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                feed_id: feed_id.into(),
                project: project.into(),
            }
        }
    }
    pub mod get_retention_policy {
        use super::models;
        type Response = models::FeedRetentionPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/retentionpolicies",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedRetentionPolicy =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod set_retention_policy {
        use super::models;
        type Response = models::FeedRetentionPolicy;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::FeedRetentionPolicy,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/retentionpolicies",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FeedRetentionPolicy =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
    pub mod delete_retention_policy {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) feed_id: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/packaging/Feeds/{}/retentionpolicies",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.feed_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod provenance {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Creates a session, a wrapper around a feed that can store additional metadata on the packages published to it."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `organization`: The name of the Azure DevOps organization."]
        #[doc = "* `body`: The feed and metadata for the session"]
        #[doc = "* `protocol`: The protocol that the session will target"]
        #[doc = "* `project`: Project ID or project name"]
        pub fn create_session(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::SessionRequest>,
            protocol: impl Into<String>,
            project: impl Into<String>,
        ) -> create_session::Builder {
            create_session::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                protocol: protocol.into(),
                project: project.into(),
            }
        }
    }
    pub mod create_session {
        use super::models;
        type Response = models::SessionResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::SessionRequest,
            pub(crate) protocol: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/{}/{}/_apis/provenance/session/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.project,
                            &this.protocol
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        if let Some(auth_header) = this
                            .client
                            .token_credential()
                            .http_authorization_header(&this.client.scopes)
                            .await?
                        {
                            req.insert_header(azure_core::headers::AUTHORIZATION, auth_header);
                        }
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "7.1-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SessionResponse =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code,
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
