import { ReactNode } from "react";

interface PageProps {
  title: ReactNode;
  subtitle: ReactNode;
  actions: ReactNode;
  children: ReactNode;
}

export const Page = ({ title, subtitle, actions, children }: PageProps) => (
  <div className="flex flex-col gap-12">
    <div className="flex flex-col gap-6 lg:flex-row lg:gap-0 lg:items-start justify-between">
      <div className="flex flex-col">
        <h1 className="text-4xl">{title}</h1>
        {subtitle}
      </div>
      {actions}
    </div>
    {children}
  </div>
);

interface SectionProps {
  title: string;
  icon: ReactNode;
  actions: ReactNode;
  children: ReactNode;
}

export const Section = ({ title, icon, actions, children }: SectionProps) => (
  <div className="flex flex-col gap-2">
    <div className="flex items-start justify-between min-h-[40px]">
      <div className="flex items-center gap-2 text-muted-foreground">
        {icon}
        <h2 className="text-xl">{title}</h2>
      </div>
      {actions}
    </div>
    {children}
  </div>
);
