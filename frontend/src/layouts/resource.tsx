import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { ReactNode } from "react";

interface ResourceProps {
  title: ReactNode;
  info: ReactNode;
  actions: ReactNode;
  tabs: { title: string; component: ReactNode }[];
}

export const Resource = ({ title, info, actions, tabs }: ResourceProps) => (
  <Tabs defaultValue={tabs[0].title}>
    <div className="flex flex-col w-full gap-12">
      <div className="flex flex-col lg:flex-row gap-2 justify-between">
        <div>
          <div className="text-4xl">{title}</div>
          <h2 className="text-xl">{info}</h2>
        </div>
        <div className="flex gap-4">{actions}</div>
      </div>
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
    </div>
  </Tabs>
);
