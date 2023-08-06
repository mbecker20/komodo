import { ReactNode } from "react";
import { Page } from "./page";

interface ResourceProps {
  title: ReactNode;
  info: ReactNode;
  actions: ReactNode;
  children: ReactNode;
}

export const Resource = ({ title, info, actions, children }: ResourceProps) => (
  <Page
    title={title}
    subtitle={<h2 className="text-md">{info}</h2>}
    actions={actions}
  >
    {children}
  </Page>
);
