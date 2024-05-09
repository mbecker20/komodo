# permissioning resources

All monitor resources (servers, builds, deployment) have independant permission tables to allow for users to have granular access to these resources. By default, users do not see any resources until they are given at least read permissions.

## permission levels

There are 4 levels of permissions a user can have on a resource:

 1. **None**. This is the lowest permission level, and means the user will not have any access to this resource. They will not see it in the GUI, and it will not show up if the user queries the core API directly. All attempts to view or update the resource will be blocked.

 2. **Read**. This is the first permission level that grants any access. It will enable the user to see the resource in the GUI, read the configuration, and see any logs. Any attempts to update configuration or trigger any action will be blocked.

 3. **Execute**. This level will allow the user to execute actions on the resource, like send a build command or trigger a redeploy. The user will still be blocked from updating configuration on the resource.

 4. **Write**. The user has full access to the resource, they can execute any actions, update the configuration, and delete the resource.

## Administration

Users can be given admin priviledges by accessing the monitor MongoDB and setting ```admin: true``` on the intended user document. These users have unrestricted access to all monitor resources, like servers, builds, and deployments. Additionally, only these users can update other (non-admin) user's permissions on resources, an action not available to regular users even with **Update** level permissions.

Monitor admins are responsible for managing user accounts as well. When a user logs into monitor for the first time, they will not immediately be granted access. An admin must first **enable** the user, which can be done from the 'manage users' page (found in the user dropdown menu in the topbar). Users can also be **disabled** by an admin at any time, which blocks all their access to the GUI and API. 

Users also have some configurable global permissions, these are:

 - create server permission
 - create build permission

Only users with these permissions (as well as admins) can add additional servers to monitor, and can create additional builds, respectively.