import { ReactNode } from "react";
import { Page } from "./page";
import { Outlet } from "react-router-dom";

interface ResourceProps {
  title: ReactNode;
  info: ReactNode;
  actions: ReactNode;
}

export const Resource = ({ title, info, actions }: ResourceProps) => (
  <Page
    title={<h1 className="text-4xl">{title}</h1>}
    subtitle={<h2 className="text-lg">{info}</h2>}
    actions={actions}
    content={<Outlet />}
  />
);
