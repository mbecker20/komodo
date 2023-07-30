import { Config } from "@components/config/Config";
import { useRead } from "@hooks";
import { Types } from "@monitor/client";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const BuildConfig = () => {
  const id = useParams().buildId;
  const build = useRead("GetBuild", { id });
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  if (build.data?.config) {
    return (
      <Config
        config={build.data?.config as any}
        update={update}
        set={set}
      />
    );
  } else {
    // loading
    return null;
  }
};
