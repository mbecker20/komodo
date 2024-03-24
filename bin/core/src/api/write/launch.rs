use anyhow::anyhow;
use async_trait::async_trait;
use monitor_client::{
  api::write::{self, LaunchServer, LaunchServerConfig},
  entities::{
    server::PartialServerConfig,
    update::{ResourceTarget, Update},
    user::User,
    Operation,
  },
};
use resolver_api::Resolve;

use crate::{
  cloud::aws::launch_ec2_instance,
  helpers::{add_update, make_update, update_update},
  state::State,
};

#[async_trait]
impl Resolve<LaunchServer, User> for State {
  async fn resolve(
    &self,
    LaunchServer { name, config }: LaunchServer,
    user: User,
  ) -> anyhow::Result<Update> {
    if !user.admin {
      return Err(anyhow!("only admins can launch servers"));
    }
    let mut update = make_update(
      ResourceTarget::System("system".to_string()),
      Operation::LaunchServer,
      &user,
    );
    update
      .push_simple_log("launching server", format!("{:#?}", config));
    update.id = add_update(update.clone()).await?;
    match config {
      LaunchServerConfig::Aws(config) => {
        let region = config.region.clone();
        let instance = launch_ec2_instance(&name, config).await;
        if let Err(e) = &instance {
          update.push_error_log(
            "launch server",
            format!("failed to launch aws instance\n\n{e:#?}"),
          );
          update.finalize();
          update_update(update.clone()).await?;
          return Ok(update);
        }
        let instance = instance.unwrap();
        update.push_simple_log(
          "launch server",
          format!(
            "successfully launched server {name} on ip {}",
            instance.ip
          ),
        );
        let _ = self
          .resolve(
            write::CreateServer {
              name,
              config: PartialServerConfig {
                address: format!("http://{}:8000", instance.ip)
                  .into(),
                region: region.into(),
                ..Default::default()
              },
            },
            user,
          )
          .await;
      }
    }
    update.finalize();
    update_update(update.clone()).await?;
    Ok(update)
  }
}
