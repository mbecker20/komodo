# Permissioning

Komodo has a granular, layer-based permissioning system to provide non-admin users access only to intended Resources.

## User Groups

While Komodo can assign permissions to specific users directly, it is recommended to instead **create User Groups and assign permissions to them**, as if they were a user.

Users can then be **added to multiple User Groups** and they **inherit the group's permissions**, similar to linux permissions.

For permissioning at scale, users can define [**User Groups in Resource Syncs**](/docs/sync-resources#user-group).

## Permission Levels

There are 4 permission levels a user / group can be given on a Resource:

 1. **None**. The user will not have any access to the resource. The user **will not see it in the GUI, and it will not show up if the user queries the Komodo API directly**. All attempts to view or update the resource will be blocked. This is the default for non-admins, unless using `KOMODO_TRANSPARENT_MODE=true`.

 2. **Read**. This is the first permission level that grants any access. It will enable the user to **see the resource in the GUI, read the configuration, and see any logs**. Any attempts to update configuration or trigger any action **will be blocked**.  Using `KOMODO_TRANSPARENT_MODE=true` will make this level the base level on all resources, for all users.

 3. **Execute**. This level will allow the user to execute actions on the resource, **like send a build command** or **trigger a redeploy**. The user will still be blocked from updating configuration on the resource.

 4. **Write**. The user has full access to the resource, **they can execute any actions, update the configuration, and delete the resource**.

## Global permissions

Users or User Groups can be given a base permission level on all Resources of a particular type, such as Stack.
In TOML form, this looks like:

```toml
[[user_group]]
name = "groupo"
users = ["mbecker20", "karamvirsingh98"]
all.Build = "Execute" # <- Group members can run all builds (but not update config),
all.Stack = "Read"    # <- And see all Stacks / logs (not deploy / update).
```

A user / group can still be given a greater permission level on select resources:

```toml
permissions = [
  { target.type = "Stack", target.id = "my-stack", level = "Execute" },
  # Use regex to match multiple resources, for example give john execute on all of their Stacks
  { target.type = "Stack", target.id = "\\^john-(.+)$\\", level = "Execute" },
]
```

## Administration

Users can be given Admin priviledges by a `Super Admin` (only the first user is given this status, set with `super_admin: true` on a User document in database). Super admins will see the "Make Admin" button when on a User page `/users/${user_id}`.

These users have unrestricted access to all Komodo Resources. Additionally, these users can update other (non-admin) user's permissions on resources.

Komodo admins are responsible for managing user accounts as well. When a user logs into Komodo for the first time, they will not immediately be granted access (this can changed with `KOMODO_ENABLE_NEW_USERS=true`). An admin must first **enable** the user, which can be done from the `Users` tab on `Settings` page. Users can also be **disabled** by an admin at any time, which blocks all their access to the GUI and API. 

Users also have some configurable global permissions, these are:

 - create server permission
 - create build permission

Only users with these permissions (as well as admins) can add additional servers to Komodo, and can create additional builds, respectively.