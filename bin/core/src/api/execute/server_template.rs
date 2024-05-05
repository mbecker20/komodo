use axum::async_trait;
use monitor_client::{
  api::{execute::LaunchServer, write::CreateServer},
  entities::{
    permission::PermissionLevel,
    server::PartialServerConfig,
    server_template::{ServerTemplate, ServerTemplateConfig},
    update::Update,
    user::User,
    Operation,
  },
};
use resolver_api::Resolve;

use crate::{
  cloud::aws::launch_ec2_instance,
  helpers::{
    resource::StateResource,
    update::{add_update, make_update, update_update},
  },
  state::State,
};

#[async_trait]
impl Resolve<LaunchServer, User> for State {
  #[instrument(name = "LaunchServer", skip(self, user))]
  async fn resolve(
    &self,
    LaunchServer {
      name,
      server_template,
    }: LaunchServer,
    user: User,
  ) -> anyhow::Result<Update> {
    let template = ServerTemplate::get_resource_check_permissions(
      &server_template,
      &user,
      PermissionLevel::Execute,
    )
    .await?;
    let mut update =
      make_update(&template, Operation::LaunchServer, &user);
    update.in_progress();
    update.push_simple_log(
      "launching server",
      format!("{:#?}", template.config),
    );
    update.id = add_update(update.clone()).await?;
    match template.config {
      ServerTemplateConfig::Aws(config) => {
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
            CreateServer {
              name,
              config: PartialServerConfig {
                address: format!("http://{}:8120", instance.ip)
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
