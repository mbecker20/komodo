import { ReactNode } from "react";

interface PageProps {
  title: ReactNode;
  subtitle: ReactNode;
  actions: ReactNode;
  content: ReactNode;
}

export const Page = ({ title, subtitle, actions, content }: PageProps) => (
  <div className="flex flex-col gap-12">
    <div className="flex flex-col gap-6 lg:flex-row lg:gap-0 justify-between">
      <div className="flex flex-col">
        {title}
        {subtitle}
      </div>
      {actions}
    </div>
    {content}
  </div>
);

interface SectionProps {
  title: string;
  icon: ReactNode;
  actions: ReactNode;
  children: ReactNode;
}

export const Section = ({ title, icon, actions, children }: SectionProps) => (
  <div className="flex flex-col">
    <div className="flex justify-between">
      <div className="flex items-center gap-2 text-muted-foreground">
        {icon}
        <h2 className="text-xl">{title}</h2>
      </div>
      {actions}
    </div>
    {children}
  </div>
);
