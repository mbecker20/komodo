import { Configuration } from "@components/config";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const BuildConfig = () => {
  const id = useParams().buildId;
  const build = useRead("GetBuild", { id }).data;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutate, isLoading } = useWrite("UpdateBuild");

  if (id && build?.config) {
    return (
      <Section
        title="Config"
        icon={<Settings className="w-4 h-4" />}
        actions={
          <div className="flex gap-4">
            <Button variant="outline" intent="warning" onClick={() => set({})}>
              <History className="w-4 h-4" />
            </Button>
            <Button
              variant="outline"
              intent="success"
              onClick={() => mutate({ config: update, id })}
            >
              <Save className="w-4 h-4" />
            </Button>
          </div>
        }
      >
        <Configuration
          config={build.config}
          loading={isLoading}
          update={update}
          set={(input) => set((update) => ({ ...update, ...input }))}
          layout={{
            // general: ["version"],
            repo: ["repo", "branch", "github_account"],
            docker: [
              "build_path",
              "dockerfile_path",
              "docker_account",
              "docker_organization",
              "use_buildx",
            ],
            pre_build: ["pre_build"],
            build_args: ["build_args"],
            extra_args: ["extra_args"],
          }}
          // overrides={{
          //   pre_build: (val, set) => {},
          // }}
        />
        {/* <Config config={build?.config as any} update={update} set={set} /> */}
      </Section>
    );
  } else {
    // loading
    return null;
  }
};
