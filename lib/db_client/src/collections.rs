use anyhow::Context;
use mungos::{Collection, Mungos};
use types::{Build, Deployment, Group, Procedure, Server, SystemStatsRecord, Update, User};

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
    coll.create_index("server_id")
        .await
        .context("failed at creating server_id index")?;
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
    coll.create_index("server_id")
        .await
        .context("failed at creating server_id index")?;
    Ok(coll)
}

pub async fn updates_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Update>> {
    let coll = mungos.collection(db_name, "updates");
    coll.create_index("target.type")
        .await
        .context("failed at creating target type index")?;
    coll.create_index("target.id")
        .await
        .context("failed at creating target id index")?;
    coll.create_index("start_ts")
        .await
        .context("failed at creating start_ts index")?;
    coll.create_index("end_ts")
        .await
        .context("failed at creating start_ts index")?;
    coll.create_index("operator")
        .await
        .context("failed at creating operator index")?;
    coll.create_index("success")
        .await
        .context("failed at creating success index")?;
    coll.create_index("status")
        .await
        .context("failed at creating status index")?;
    coll.create_index("operation")
        .await
        .context("failed at creating success index")?;
    Ok(coll)
}

pub async fn procedures_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Procedure>> {
    let coll = mungos.collection(db_name, "procedures");
    coll.create_unique_index("name")
        .await
        .context("failed at creating entity_id index")?;
    Ok(coll)
}

pub async fn groups_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<Group>> {
    let coll = mungos.collection(db_name, "groups");
    coll.create_unique_index("name")
        .await
        .context("failed at creating name index")?;
    Ok(coll)
}

pub async fn server_stats_collection(
    mungos: &Mungos,
    db_name: &str,
) -> anyhow::Result<Collection<SystemStatsRecord>> {
    let coll = mungos.collection(db_name, "stats");
    coll.create_index("server_id")
        .await
        .context("failed at creating server_id index")?;
    coll.create_index("ts")
        .await
        .context("failed at creating ts index")?;
    Ok(coll)
}
