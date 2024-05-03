# adding the server to monitor

The easiest way to add the server is with the GUI. On the home page, click the ```+``` button to the right of the server search bar, configure the name and address of the server. The address is the full http/s url to the periphery server, eg ```http://12.34.56.78:8000```.

Once it is added, you can use access the GUI to modify some config, like the alerting thresholds for cpu, memory and disk usage. A server can also be temporarily disabled, this will prevent alerting if it goes offline.

Since no state is stored on the periphery servers, you can easily redirect all deployments to be hosted on a different server. Just update the address to point to the new server.