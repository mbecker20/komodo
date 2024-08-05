import { PageXlRow, Section } from "@components/layouts";
import { ResourceLink } from "@components/resources/common";
import { useStack } from "@components/resources/stack";
import {
  PauseUnpauseStack,
  RestartStack,
  StartStopStack,
} from "@components/resources/stack/actions";
import {
  bg_color_class_by_intention,
  deployment_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import { useRead, useSetTitle } from "@lib/hooks";
import { cn, has_minimum_permissions } from "@lib/utils";
import { Types } from "@monitor/client";
import { Box, Clapperboard, Layers2 } from "lucide-react";
import { useParams } from "react-router-dom";
import { StackServiceLogs } from "./log";
import { ResourceUpdates } from "@components/updates/resource";

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
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Stack", id: stack_id },
  }).data;
  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );
  const services = useRead("ListStackServices", { stack: stack_id }).data;
  const container = services?.find((s) => s.service === service)?.container;
  const state = container?.state ?? Types.DeploymentState.Unknown;
  const intention = deployment_state_intention(state);
  const bg_color = bg_color_class_by_intention(intention);
  const stroke_color = stroke_color_class_by_intention(intention);

  return (
    <PageXlRow
      wrapSize="lg"
      title={service}
      icon={<Layers2 className={cn("w-8 h-8", stroke_color)} />}
      titleRight={
        // <Card className={cn("w-fit", bg_color)}>
        //   <CardHeader className="py-0 px-2">
        //     {snake_case_to_upper_space_case(state).toUpperCase()}
        //   </CardHeader>
        // </Card>
        <div className="flex flex-wrap gap-4 items-center">
          <p
            className={cn(
              "p-1 w-fit text-[10px] text-white rounded-md",
              bg_color
            )}
          >
            {snake_case_to_upper_space_case(state).toUpperCase()}
          </p>
          {container?.status && <div>{container?.status}</div>}
        </div>
      }
      subtitle={
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
      }
    >
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
    </PageXlRow>
  );
};
