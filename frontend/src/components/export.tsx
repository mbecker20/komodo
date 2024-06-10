import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { FileDown, Loader2 } from "lucide-react";
import { useState } from "react";
import { CopyButton } from "./util";

export const ExportButton = ({
  targets,
  user_groups,
  tags,
  include_variables,
}: {
  targets?: Types.ResourceTarget[];
  user_groups?: string[];
  tags?: string[];
  include_variables?: boolean;
}) => {
  const [open, setOpen] = useState(false);
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" className="flex gap-2 items-center">
          Toml
          <FileDown className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent className="min-w-[600px]">
        <DialogHeader>
          <DialogTitle>Export to Toml</DialogTitle>
        </DialogHeader>
        {targets || user_groups || include_variables ? (
          <ExportTargetsLoader
            targets={targets}
            user_groups={user_groups}
            include_variables={include_variables}
          />
        ) : (
          <ExportAllLoader tags={tags} />
        )}
      </DialogContent>
    </Dialog>
  );
};

const ExportTargetsLoader = ({
  targets,
  user_groups,
  include_variables,
}: {
  targets?: Types.ResourceTarget[];
  user_groups?: string[];
  include_variables?: boolean;
}) => {
  const { data, isPending } = useRead("ExportResourcesToToml", {
    targets: targets ? targets : [],
    user_groups: user_groups ? user_groups : [],
    include_variables,
  });
  return <ExportPre loading={isPending} content={data?.toml} />;
};

const ExportAllLoader = ({
  tags,
  include_variables,
}: {
  tags?: string[];
  include_variables?: boolean;
}) => {
  const { data, isPending } = useRead("ExportAllResourcesToToml", {
    tags,
    include_variables,
  });
  return <ExportPre loading={isPending} content={data?.toml} />;
};

const ExportPre = ({
  loading,
  content,
}: {
  loading: boolean;
  content: string | undefined;
}) => {
  return (
    <div className="relative flex justify-center w-full">
      {loading && <Loader2 className="w-8 h-8 animate-spin" />}
      <div className="overflow-y-scroll max-h-[80vh] w-full">
        <pre className="h-fit w-full">{content}</pre>
      </div>
      <CopyButton content={content} className="absolute top-4 right-4" />
    </div>
  );
};
