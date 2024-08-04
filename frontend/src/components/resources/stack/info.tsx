import { Section } from "@components/layouts"
import { ReactNode } from "react";
import { useRead } from "@lib/hooks";

export const StackInfo = ({ id, titleOther }: { id: string; titleOther: ReactNode }) => {
	const stack = useRead("GetStack", { stack: id }).data;
	return (
    <Section titleOther={titleOther}>
      <div>file missing: {stack?.info?.file_missing ? "true" : "false"}</div>
      <pre>deployed contents: {stack?.info?.deployed_contents}</pre>
      <pre>deployed json: {stack?.info?.deployed_json}</pre>
      <div>deployed message: {stack?.info?.deployed_message}</div>

      <pre>latest contents: {stack?.info?.remote_contents}</pre>
      <pre>latest json: {stack?.info?.latest_json}</pre>
      <div>latest message: {stack?.info?.latest_message}</div>
    </Section>
  );
}