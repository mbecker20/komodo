import { Card, CardHeader, CardTitle } from "@ui/card";
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
    <div className="flex flex-col w-full gap-4">
      <Card>
        <CardHeader className="gap-4">
          <div className="flex flex-col">
            <div className="flex flex-col gap-2 md:flex-row md:justify-between">
              <CardTitle className="text-3xl">{title}</CardTitle>
              {info}
            </div>
          </div>
          <div className="flex gap-4 flex-col md:flex-row md:justify-between">
            <TabsList>
              {tabs.map(({ title }) => (
                <TabsTrigger key={title} value={title}>
                  {title}
                </TabsTrigger>
              ))}
            </TabsList>
            <div className="flex gap-4 place-self-end md:place-self-auto">
              {actions}
            </div>
          </div>
        </CardHeader>
      </Card>
      {tabs.map((t, i) => (
        <TabsContent key={i} value={t.title}>
          {t.component}
        </TabsContent>
      ))}
    </div>
  </Tabs>
);
