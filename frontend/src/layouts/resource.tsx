import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { ReactNode } from "react";
import { Page } from "./page";

interface ResourceProps {
  title: ReactNode;
  info: ReactNode;
  actions: ReactNode;
  tabs: { title: string; component: ReactNode }[];
}

export const Resource = ({ title, info, actions, tabs }: ResourceProps) => (
  <Tabs defaultValue={tabs[0].title}>
    <Page
      title={<h1 className="text-4xl">{title}</h1>}
      subtitle={<h2 className="text-lg">{info}</h2>}
      actions={actions}
      content={
        <div className="flex flex-col gap-2">
          <TabsList className=" w-fit">
            {tabs.map(({ title }) => (
              <TabsTrigger key={title} value={title}>
                {title}
              </TabsTrigger>
            ))}
          </TabsList>
          {tabs.map((t, i) => (
            <TabsContent key={i} value={t.title}>
              {t.component}
            </TabsContent>
          ))}
        </div>
      }
    />
  </Tabs>
);
