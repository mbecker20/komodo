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
    title={<h1 className="text-4xl">{title}</h1>}
    subtitle={<h2 className="text-md">{info}</h2>}
    actions={actions}
    content={children}
  />
);
