import { Page, Section } from "@components/layouts";
import { DeploymentTable } from "@components/resources/deployment";
import { ServerIconComponent, ServerInfo } from "@components/resources/server";
import { TagsFilter, TagsWithBadge, useTagsFilter } from "@components/tags";
import { useRead } from "@lib/hooks";
import { Button } from "@ui/button";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { useState } from "react";
import { Link } from "react-router-dom";

export const Tree = () => {
  const tags = useTagsFilter();
  const servers = useRead("ListServers", { query: { tags } }).data;
  return (
    <Page title="Tree" actions={<TagsFilter />}>
      <Section title="">
        {servers?.map((server) => (
          <Server key={server.id} id={server.id} />
        ))}
      </Section>
    </Page>
  );
};

const Server = ({ id }: { id: string }) => {
  const [open, setOpen] = useState(false);
  const server = useRead("ListServers", {}).data?.find(
    (server) => server.id === id
  );
  const deployments = useRead("ListDeployments", {}).data?.filter(
    (deployment) => deployment.info.server_id === id
  );
  return (
    <>
      <Card
        className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors cursor-pointer"
        onClick={() => setOpen(!open)}
      >
        <CardHeader className="p-4 flex-row justify-between items-center">
          <div>
            <CardTitle>{server?.name}</CardTitle>
            <CardDescription>Server</CardDescription>
          </div>
          <div className="flex gap-3 justify-between items-center">
            <TagsWithBadge tag_ids={server?.tags} />
            {server?.id && <ServerInfo id={server.id} showRegion={false} />}
            <Link to={`/servers/${server?.id}`}>
              <Button variant="outline">
                <ServerIconComponent id={server?.id} />
              </Button>
            </Link>
          </div>
        </CardHeader>
      </Card>
      {open && <DeploymentTable deployments={deployments} />}
    </>
  );
};
