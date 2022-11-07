use anyhow::Context;
use mungos::{Collection, Mungos};
use types::{Build, Deployment, Procedure, Server, Update, User};

pub async fn users_collection(mungos: &Mungos, db_name: &str) -> anyhow::Result<Collection<User>> {
    let coll = mungos.collection(db_name, "users");
    coll.create_unique_index("username")
        .await
        .context("failed at creating username index")?;
    Ok(coll)
}

pub async fn servers_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Server>> {
    let coll = mungos.collection(db_name, "servers");
    coll.create_unique_index("name")
        .await
        .context("failed at creating name index")?;
    coll.create_index("permissions")
        .await
        .context("failed at creating permissions index")?;
    Ok(coll)
}

pub async fn deployments_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Deployment>> {
    let coll = mungos.collection(db_name, "deployments");
    coll.create_unique_index("name")
        .await
        .context("failed at creating name index")?;
    coll.create_index("permissions")
        .await
        .context("failed at creating permissions index")?;
    Ok(coll)
}

pub async fn builds_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Build>> {
    let coll = mungos.collection(db_name, "builds");
    coll.create_unique_index("name")
        .await
        .context("failed at creating name index")?;
    coll.create_index("permissions")
        .await
        .context("failed at creating permissions index")?;
    Ok(coll)
}

pub async fn updates_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Update>> {
    let coll = mungos.collection(db_name, "updates");
    coll.create_index("entity_id")
        .await
        .context("failed at creating entity_id index")?;
    coll.create_index("ts")
        .await
        .context("failed at creating ts index")?;
    coll.create_index("operator")
        .await
        .context("failed at creating operator index")?;
    Ok(coll)
}

pub async fn procedures_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Procedure>> {
    let coll = mungos.collection(db_name, "procedures");
    coll.create_index("name")
        .await
        .context("failed at creating entity_id index")?;
    coll.create_index("permissions")
        .await
        .context("failed at creating permissions index")?;
    Ok(coll)
}
