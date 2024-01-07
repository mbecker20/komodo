use async_trait::async_trait;
use monitor_client::{
  api::read::{FindResources, FindResourcesResponse},
  entities::{
    build, deployment, repo, server,
    update::ResourceTargetVariant::{self, *},
  },
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser, helpers::resource::StateResource, state::State,
};

const FIND_RESOURCE_TYPES: [ResourceTargetVariant; 4] =
  [Server, Build, Deployment, Repo];

#[async_trait]
impl Resolve<FindResources, RequestUser> for State {
  async fn resolve(
    &self,
    FindResources { query, resources }: FindResources,
    user: RequestUser,
  ) -> anyhow::Result<FindResourcesResponse> {
    let mut res = FindResourcesResponse::default();
    let resource_types = resources
      .map(|rs| {
        rs.into_iter()
          .filter(|r| !matches!(r, System | Builder | Alerter))
          .collect()
      })
      .unwrap_or(FIND_RESOURCE_TYPES.to_vec());
    for resource_type in resource_types {
      match resource_type {
        Server => {
          res.servers = <State as StateResource<
                        server::Server,
                    >>::list_resources_for_user(
                        self,
                        query.clone(),
                        &user,
                    )
                    .await?;
        }
        Deployment => {
          res.deployments = <State as StateResource<
            deployment::Deployment,
          >>::list_resources_for_user(
            self, query.clone(), &user
          )
          .await?;
        }
        Build => {
          res.builds = <State as StateResource<
                        build::Build,
                    >>::list_resources_for_user(
                        self,
                        query.clone(),
                        &user,
                    )
                    .await?;
        }
        Repo => {
          res.repos = <State as StateResource<
                        repo::Repo,
                    >>::list_resources_for_user(
                        self,
                        query.clone(),
                        &user,
                    )
                    .await?;
        }
        _ => unreachable!(),
      }
    }

    todo!()
  }
}

// #[async_trait]
// impl Resolve<FindResources, RequestUser> for State {
//     async fn resolve(
//         &self,
//         FindResources { search, tags }: FindResources,
//         user: RequestUser,
//     ) -> anyhow::Result<FindResourcesResponse> {
//         let SeperateTags {
//             resource_types,
//             server_ids,
//             custom_tag_ids,
//         } = seperate_tags(tags);

//         let mut query = doc! {
//             "name": { "$regex": search }
//         };

//         if !user.is_admin {
//             query.insert(
//                 format!("permissions.{}", user.id),
//                 doc! { "$in": ["read", "execute", "update"] },
//             );
//         }

//         if !custom_tag_ids.is_empty() {
//             query.insert("tags", doc! { "$all": custom_tag_ids });
//         }

//         let mut response = FindResourcesResponse::default();

//         for resource_type in resource_types {
//             match resource_type {
//                 Server => {
//                     let servers = if server_ids.is_empty() {
//                         self.db.servers.get_some(query.clone(), None).await?
//                     } else {
//                         let server_ids = server_ids
//                             .iter()
//                             .map(|id| {
//                                 ObjectId::from_str(id)
//                                     .context("failed to parse server id as ObjectId")
//                             })
//                             .collect::<anyhow::Result<Vec<_>>>()?;
//                         let mut query = query.clone();
//                         query.insert("_id", doc! { "$in": server_ids });
//                         self.db.servers.get_some(query, None).await?
//                     };
//                     for server in servers {
//                         let status = self
//                             .server_status_cache
//                             .get(&server.id)
//                             .await
//                             .map(|s| s.status)
//                             .unwrap_or_default();
//                         let item = ServerListItem {
//                             status,
//                             id: server.id,
//                             name: server.name,
//                             tags: server.tags,
//                         };
//                         response.servers.push(item);
//                     }
//                 }
//                 Deployment => {
//                     let mut query = query.clone();

//                     if !server_ids.is_empty() {
//                         query.insert("config.server_id", doc! { "$in": &server_ids });
//                     }

//                     let deployments = self
//                         .db
//                         .deployments
//                         .get_some(query, None)
//                         .await?
//                         .into_iter()
//                         .filter(|d| d.get_user_permissions(&user.id) > PermissionLevel::Read);

//                     for deployment in deployments {
//                         let status = self.deployment_status_cache.get(&deployment.id).await;
//                         let item = DeploymentListItem {
//                             id: deployment.id,
//                             name: deployment.name,
//                             tags: deployment.tags,
//                             state: status.as_ref().map(|s| s.state).unwrap_or_default(),
//                             status: status.as_ref().and_then(|s| {
//                                 s.container.as_ref().and_then(|c| c.status.to_owned())
//                             }),
//                             image: String::new(),
//                             server_id: String::new(),
//                             build_id: None,
//                         };
//                         response.deployments.push(item);
//                     }
//                 }
//                 Build => {
//                     let mut query = query.clone();

//                     if !server_ids.is_empty() {
//                         query.insert(
//                             "config.builder.params.server_id",
//                             doc! { "$in": &server_ids },
//                         );
//                     }

//                     let builds = self
//                         .db
//                         .builds
//                         .get_some(query, None)
//                         .await?
//                         .into_iter()
//                         .filter(|d| d.get_user_permissions(&user.id) > PermissionLevel::Read);

//                     for build in builds {
//                         let item = BuildListItem {
//                             id: build.id,
//                             name: build.name,
//                             tags: build.tags,
//                             last_built_at: build.last_built_at,
//                             version: build.config.version,
//                         };
//                         response.builds.push(item);
//                     }
//                 }
//                 Repo => {
//                     let mut query = query.clone();

//                     if !server_ids.is_empty() {
//                         query.insert("config.server_id", doc! { "$in": &server_ids });
//                     }

//                     let repos = self
//                         .db
//                         .repos
//                         .get_some(query, None)
//                         .await?
//                         .into_iter()
//                         .filter(|d| d.get_user_permissions(&user.id) > PermissionLevel::Read);

//                     for repo in repos {
//                         let item = RepoListItem {
//                             id: repo.id,
//                             name: repo.name,
//                             tags: repo.tags,
//                             last_pulled_at: repo.last_pulled_at,
//                         };
//                         response.repos.push(item);
//                     }
//                 }
//                 _ => return Err(anyhow!("{resource_type} is not compatible with this route")),
//             }
//         }

//         Ok(response)
//     }
// }

// #[derive(Default)]
// struct SeperateTags {
//     resource_types: Vec<ResourceTargetVariant>,
//     server_ids: Vec<String>,
//     custom_tag_ids: Vec<String>,
// }

// fn seperate_tags(tags: Vec<Tag>) -> SeperateTags {
//     let mut seperated = SeperateTags::default();

//     for tag in tags {
//         match tag {
//             Tag::Custom { tag_id } => seperated.custom_tag_ids.push(tag_id),
//             Tag::Server { server_id } => seperated.server_ids.push(server_id),
//             Tag::ResourceType { resource } => {
//                 if !matches!(resource, Builder | Alerter | System,)
//                     && !seperated.resource_types.contains(&resource)
//                 {
//                     seperated.resource_types.push(resource);
//                 }
//             }
//         }
//     }

//     if seperated.resource_types.is_empty() {
//         seperated.resource_types = FIND_RESOURCE_TYPES.to_vec();
//     }

//     seperated
// }
