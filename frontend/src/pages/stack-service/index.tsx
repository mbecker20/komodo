import { Section } from "@components/layouts";
import {
  ResourceDescription,
  ResourceLink,
} from "@components/resources/common";
import { useStack } from "@components/resources/stack";
import {
  PauseUnpauseStack,
  RestartStack,
  StartStopStack,
} from "@components/resources/stack/actions";
import {
  deployment_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { useRead, useSetTitle } from "@lib/hooks";
import { cn, has_minimum_permissions } from "@lib/utils";
import { Types } from "@monitor/client";
import { Box, ChevronLeft, Clapperboard, Layers2 } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";
import { StackServiceLogs } from "./log";
import { ResourceUpdates } from "@components/updates/resource";
import { Button } from "@ui/button";
import { ExportButton } from "@components/export";
import { AddTags, ResourceTags } from "@components/tags";
import { StatusBadge } from "@components/util";

type IdServiceComponent = React.FC<{ id: string; service?: string }>;

const Actions: { [action: string]: IdServiceComponent } = {
  RestartStack,
  PauseUnpauseStack,
  StartStopStack,
};

export const StackServicePage = () => {
  const { id: stack_id, service } = useParams() as {
    id: string;
    service: string;
  };
  const stack = useStack(stack_id);
  useSetTitle(`${stack?.name} | ${service}`);
  const nav = useNavigate();
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Stack", id: stack_id },
  }).data;
  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );
  const canWrite = has_minimum_permissions(perms, Types.PermissionLevel.Write);
  const services = useRead("ListStackServices", { stack: stack_id }).data;
  const container = services?.find((s) => s.service === service)?.container;
  const state = container?.state ?? Types.DeploymentState.Unknown;
  const intention = deployment_state_intention(state);
  const stroke_color = stroke_color_class_by_intention(intention);

  return (
    <div className="flex flex-col gap-16">
      <div className="flex flex-col gap-4">
        <div className="flex items-center justify-between mb-4">
          <Button
            className="gap-2"
            variant="secondary"
            onClick={() => nav("/stacks/" + stack_id)}
          >
            <ChevronLeft className="w-4" /> Back
          </Button>
          <ExportButton targets={[{ type: "Stack", id: stack_id }]} />
        </div>

        <div className="grid lg:grid-cols-2 gap-4">
          <div className="flex items-center gap-4">
            <div className="mt-1">
              <Layers2 className={cn("w-8 h-8", stroke_color)} />
            </div>
            <h1 className="text-3xl">{service}</h1>
            <div className="flex flex-wrap gap-4 items-center">
              <StatusBadge text={state} intent={intention} />
              {container?.status && <div>{container?.status}</div>}
            </div>
          </div>

          <div className="flex items-center gap-2 lg:justify-self-end">
            <p className="text-sm text-muted-foreground">Description: </p>
            <ResourceDescription
              type="Stack"
              id={stack_id}
              disabled={!canWrite}
            />
          </div>

          <div className="flex flex-wrap gap-4 items-center text-muted-foreground">
            <ResourceLink type="Stack" id={stack_id} />
            {stack?.info.server_id && (
              <>
                |
                <ResourceLink type="Server" id={stack.info.server_id} />
              </>
            )}
            {container && container?.name !== service && (
              <>
                |
                <div className="flex gap-2 items-center">
                  <Box className="w-4 h-4" />
                  {container.name}
                </div>
              </>
            )}
          </div>

          <div className="flex items-center gap-2 h-7 lg:justify-self-end">
            <p className="text-sm text-muted-foreground">Tags:</p>
            <ResourceTags
              target={{ id: stack_id, type: "Stack" }}
              className="text-sm"
              disabled={!canWrite}
              click_to_delete
            />
            {canWrite && <AddTags target={{ id: stack_id, type: "Stack" }} />}
          </div>
        </div>
      </div>

      {/* Actions */}
      {canExecute && (
        <Section
          title="Actions (Service)"
          icon={<Clapperboard className="w-4 h-4" />}
        >
          <div className="flex gap-4 items-center flex-wrap">
            {Object.entries(Actions).map(([key, Action]) => (
              <Action key={key} id={stack_id} service={service} />
            ))}
          </div>
        </Section>
      )}

      {/* Updates */}
      <ResourceUpdates type="Stack" id={stack_id} />

      {/* Logs */}
      <div className="pt-4">
        <StackServiceLogs id={stack_id} service={service} />
      </div>
    </div>
  );
};
