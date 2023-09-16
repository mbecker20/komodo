use anyhow::anyhow;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        server::PartialServerConfig,
        update::{ResourceTarget, Update},
        Operation,
    },
    requests::write::{self, LaunchServer, LaunchServerConfig},
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, helpers::make_update, state::State};

#[async_trait]
impl Resolve<LaunchServer, RequestUser> for State {
    async fn resolve(
        &self,
        LaunchServer { name, config }: LaunchServer,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if !user.is_admin {
            return Err(anyhow!("only admins can launch servers"));
        }
        let mut update = make_update(
            ResourceTarget::System("system".to_string()),
            Operation::LaunchServer,
            &user,
        );
        update.push_simple_log(
            "launching server",
            format!("{:#?}", config),
        );
        update.id = self.add_update(update.clone()).await?;
        match config {
            LaunchServerConfig::Aws(config) => {
                let region = config.region.clone();
                let instance =
                    self.launch_ec2_instance(&name, config).await;
                if let Err(e) = &instance {
                    update.push_error_log(
                        "launch server",
                        format!(
                            "failed to launch aws instance\n\n{e:#?}"
                        ),
                    );
                    update.finalize();
                    self.update_update(update.clone()).await?;
                    return Ok(update);
                }
                let instance = instance.unwrap();
                update.push_simple_log(
                    "launch server",
                    format!("successfully launched server {name} on ip {}", instance.ip),
                );
                let _ = self
                    .resolve(
                        write::CreateServer {
                            name,
                            config: PartialServerConfig {
                                address: format!(
                                    "http://{}:8000",
                                    instance.ip
                                )
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
        self.update_update(update.clone()).await?;
        Ok(update)
    }
}
