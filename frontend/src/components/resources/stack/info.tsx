import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { useRead } from "@lib/hooks";
import { useStack } from ".";

export const StackInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const stack = useRead("GetStack", { stack: id }).data;
  const projects = useRead("ListComposeProjects", {
    server: stack?.config?.server_id!,
  }).data;
  const info = useStack(id)?.info;
  return (
    <Section titleOther={titleOther}>
      <pre>
        {projects?.map((project) => (
          <pre>{JSON.stringify(project, undefined, 2)}</pre>
        ))}
      </pre>
      <div>project missing: {info?.project_missing ? "true" : "false"}</div>
      <pre>
        deployed contents:{" "}
        {stack?.info?.deployed_contents?.map((content) => (
          <div>
            path: {content.path}
            <pre>{content.contents}</pre>
          </div>
        ))}
      </pre>
      <pre>
        deployed json:{" "}
        {stack?.info?.deployed_json?.map((content) => (
          <div>
            path: {content.path}
            <pre>{content.contents}</pre>
          </div>
        ))}
      </pre>
      <div>deployed message: {stack?.info?.deployed_message}</div>

      <pre>
        remote contents:{" "}
        {stack?.info?.remote_contents?.map((content) => (
          <div>
            path: {content.path}
            <pre>{content.contents}</pre>
          </div>
        ))}
      </pre>
      <pre>
        latest json:{" "}
        {stack?.info?.latest_json?.map((content) => (
          <div>
            path: {content.path}
            <pre>{content.contents}</pre>
          </div>
        ))}
      </pre>
      <pre>
        latest json error:{" "}
        {stack?.info?.latest_json_errors?.map((content) => (
          <div>
            path: {content.path}
            <pre>{content.contents}</pre>
          </div>
        ))}
      </pre>
      <div>latest message: {stack?.info?.latest_message}</div>
    </Section>
  );
};
