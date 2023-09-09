import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents, UsableResource } from "@types";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { AlarmClock, Factory, GitBranch, Hammer } from "lucide-react";
import { Link } from "react-router-dom";
import { Deployment } from "./deployment";
import { Server } from "./server";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find((d) => d.id === id);

const useBuild = (id?: string) =>
  useRead("ListBuilds", {}).data?.find((d) => d.id === id);

const useBuilder = (id?: string) =>
  useRead("ListBuilders", {}).data?.find((d) => d.id === id);

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const ResourceComponents: {
  [key in UsableResource]: RequiredResourceComponents;
} = {
  Alerter: {
    Name: ({ id }) => <>{useAlerter(id)?.name}</>,
    Description: ({ id }) => <>{id}</>,
    Info: ({ id }) => <>{id}</>,
    Icon: () => <AlarmClock className="w-4 h-4" />,
    Page: {},
    Actions: () => null,
    New: () => null,
  },
  Build: {
    Name: ({ id }) => <>{useBuild(id)?.name}</>,
    Description: ({ id }) => <>{id}</>,
    Info: ({ id }) => <>{id}</>,
    Icon: () => <Hammer className="w-4 h-4" />,
    Page: {},
    Actions: () => null,
    New: () => null,
  },
  Builder: {
    Name: ({ id }) => <>{useBuilder(id)?.name}</>,
    Description: ({ id }) => <>{id}</>,
    Info: ({ id }) => <>{id}</>,
    Icon: () => <Factory className="w-4 h-4" />,
    Page: {},
    Actions: () => null,
    New: () => null,
  },
  Repo: {
    Name: ({ id }) => <>{useRepo(id)?.name}</>,
    Description: ({ id }) => <>{id}</>,
    Info: ({ id }) => <>{id}</>,
    Icon: () => <GitBranch className="w-4 h-4" />,
    Page: {},
    Actions: () => null,
    New: () => null,
  },
  Deployment,
  Server,
};

export const ResourceCard = ({
  target: { type, id },
}: {
  target: Exclude<Types.ResourceTarget, { type: "System" }>;
}) => {
  const Components = ResourceComponents[type];

  return (
    <Link
      to={`/${type.toLowerCase()}s/${id}`}
      className="group hover:translate-y-[-2.5%] focus:translate-y-[-2.5%] transition-transform"
    >
      <Card className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors">
        <CardHeader className="justify-between">
          <div>
            <CardTitle>
              <Components.Name id={id} />
            </CardTitle>
            <CardDescription>
              <Components.Description id={id} />
            </CardDescription>
          </div>
          <Components.Icon id={id} />
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground">
          <Components.Info id={id} />
        </CardContent>
      </Card>
    </Link>
  );
};
