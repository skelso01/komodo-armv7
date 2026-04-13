use anyhow::{Context, anyhow};
use database::mungos::mongodb::bson::oid::ObjectId;
use komodo_client::entities::{
  builder::{Builder, BuilderConfig},
  server::Server,
  update::Update,
};

use crate::{
  cloud::BuildCleanupData,
  connection::PeripheryConnectionArgs,
  periphery::PeripheryClient,
  resource,
};

use super::periphery_client;

const BUILDER_POLL_RATE_SECS: u64 = 2;
const BUILDER_POLL_MAX_TRIES: usize = 60;

#[instrument(
  "ConnectBuilderPeriphery",
  skip_all,
  fields(
    resource_name,
    builder_id = builder.id,
    update_id = update.id
  )
)]
pub async fn connect_builder_periphery(
  _resource_name: String,
  _version: Option<komodo_client::entities::Version>,
  builder: Builder,
  _update: &mut Update,
) -> anyhow::Result<(PeripheryClient, BuildCleanupData)> {
  match builder.config {
    BuilderConfig::Url(config) => {
      if config.address.is_empty() {
        return Err(anyhow!(
          "Builder has not yet configured an address"
        ));
      }
      // TODO: Dont use builder id, or will be problems
      // with simultaneous spawned builders.
      let periphery = PeripheryClient::new(
        PeripheryConnectionArgs::from_url_builder(
          &ObjectId::new().to_hex(),
          &config,
        ),
        config.insecure_tls,
      )
      .await?;
      periphery
        .health_check()
        .await
        .context("Url Builder failed health check")?;
      Ok((periphery, BuildCleanupData::Url))
    }
    BuilderConfig::Server(config) => {
      if config.server_id.is_empty() {
        return Err(anyhow!("Builder has not configured a server"));
      }
      let server = resource::get::<Server>(&config.server_id).await?;
      let periphery = periphery_client(&server).await?;
      Ok((periphery, BuildCleanupData::Server))
    }
  }
}


#[instrument(
  "CleanupBuilderInstance",
  skip_all,
  fields(update_id = update.id)
)]
pub async fn cleanup_builder_instance(
  periphery: PeripheryClient,
  cleanup_data: BuildCleanupData,
  update: &mut Update,
) {
  match cleanup_data {
    BuildCleanupData::Server => {
      // Nothing to clean up
    }
    BuildCleanupData::Url => {
      periphery.cleanup().await;
    }
  }
}


