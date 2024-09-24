mod abi;
mod config;
pub mod pb;

use abi::auth;
use anyhow::Result;
pub use config::AppConfig;

use crm_metadata::pb::metadata_client::MetadataClient;
use crm_send::pb::notification_client::NotificationClient;
use pb::{
    crm_server::{Crm, CrmServer},
    RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest, WelcomeResponse,
};
use tonic::{async_trait, service::interceptor::InterceptedService, transport::Channel, Request};
use tracing::info;
use user_stat::pb::user_stats_client::UserStatsClient;

pub struct CrmService {
    config: AppConfig,
    user_stats: UserStatsClient<Channel>,
    notification: NotificationClient<Channel>,
    metadata: MetadataClient<Channel>,
}

#[async_trait]
impl Crm for CrmService {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<tonic::Response<WelcomeResponse>, tonic::Status> {
        let user: &auth::User = request.extensions().get().unwrap();
        info!("user: {:?}", user);
        self.welcome(request.into_inner()).await
    }
    ///  last visited or watched in X days, give them something to watch
    async fn recall(
        &self,
        _request: tonic::Request<RecallRequest>,
    ) -> Result<tonic::Response<RecallResponse>, tonic::Status> {
        todo!()
    }
    /// last watched in X days, and user still have unfinished contents
    async fn remind(
        &self,
        _request: tonic::Request<RemindRequest>,
    ) -> Result<tonic::Response<RemindResponse>, tonic::Status> {
        todo!()
    }
}

impl CrmService {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let user_stats = UserStatsClient::connect(config.server.user_stats.clone()).await?;
        let notification = NotificationClient::connect(config.server.notification.clone()).await?;
        let metadata = MetadataClient::connect(config.server.metadata.clone()).await?;
        Ok(Self {
            config,
            user_stats,
            notification,
            metadata,
        })
    }

    pub fn into_server(
        self,
    ) -> Result<InterceptedService<CrmServer<CrmService>, auth::DecodingKey>> {
        let dk = auth::DecodingKey::load(&self.config.auth.pk)?;
        Ok(CrmServer::with_interceptor(self, dk))
    }
}
