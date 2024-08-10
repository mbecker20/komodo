import { useLocalStorage, useUser } from "@lib/hooks";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { CreateVariable, Variables } from "./variables";
import { CreateTag, Tags } from "./tags";
import { UsersPage } from "./users";
import { CreateKey, Keys } from "./keys";
import { Page } from "@components/layouts";
import { useState } from "react";
import { Input } from "@ui/input";
import { ProvidersPage } from "./providers";

export const Settings = () => {
  const user = useUser().data;
  const [view, setView] = useLocalStorage("settings-view-v0", "Variables");
  const [search, setSearch] = useState("");
  const currentView =
    (view === "Users" || view === "Providers") && !user?.admin
      ? "Variables"
      : view;
  return (
    <Page>
      <Tabs
        value={currentView}
        onValueChange={setView}
        className="flex flex-col gap-6"
      >
        <div className="flex items-center justify-between">
          <TabsList className="justify-start w-fit">
            <TabsTrigger value="Variables">Variables</TabsTrigger>
            <TabsTrigger value="Tags">Tags</TabsTrigger>
            {user?.admin && (
              <TabsTrigger value="Providers">Providers</TabsTrigger>
            )}
            {user?.admin && <TabsTrigger value="Users">Users</TabsTrigger>}
            <TabsTrigger value="Api Keys">Api Keys</TabsTrigger>
          </TabsList>

          {currentView === "Variables" && <CreateVariable />}
          {currentView === "Tags" && <CreateTag />}
          {currentView === "Api Keys" && <CreateKey />}
          {currentView === "Users" && (
            <Input
              placeholder="Search"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="w-[250px]"
            />
          )}
        </div>

        <TabsContent value="Variables">
          <Variables />
        </TabsContent>
        <TabsContent value="Tags">
          <Tags />
        </TabsContent>
        {user?.admin && (
          <TabsContent value="Providers">
            <ProvidersPage />
          </TabsContent>
        )}
        {user?.admin && (
          <TabsContent value="Users">
            <UsersPage search={search} />
          </TabsContent>
        )}
        <TabsContent value="Api Keys">
          <Keys />
        </TabsContent>
      </Tabs>
    </Page>
  );
};
