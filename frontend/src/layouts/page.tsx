import { ConfirmUpdate } from "@components/config/confirm-update";
import { Button } from "@ui/button";
import { Settings, History } from "lucide-react";
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

export const ConfigLayout = ({
  content,
  children,
  onConfirm,
  onReset,
}: {
  content: any;
  children: ReactNode;
  onConfirm: () => void;
  onReset: () => void;
}) => (
  <Section
    title="Config"
    icon={<Settings className="w-4 h-4" />}
    actions={
      <div className="flex gap-4">
        <Button variant="outline" intent="warning" onClick={onReset}>
          <History className="w-4 h-4" />
        </Button>
        <ConfirmUpdate
          content={JSON.stringify(content, null, 2)}
          onConfirm={onConfirm}
        />
      </div>
    }
  >
    {children}
  </Section>
);
