# Adding Servers to Monitor

The easiest way to add servers is with the GUI. 
Navigate to the Servers page, click the New Server button, input the name, and hit create.
This will navigate to the created server, where you can configure it's address.
The address is the full http/s url to the periphery server, eg `http://12.34.56.78:8120`.

Once it is added, you can use access the GUI to modify some config, like the alerting thresholds for cpu, memory and disk usage. A server can also be temporarily disabled, this will prevent alerting if it goes offline.

Since no state is stored on the periphery servers, you can easily redirect all deployments to be hosted on a different server. Just update the address to point to the new server.