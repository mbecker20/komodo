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

export const ExportButton = ({}: {
  target?: Types.ResourceTarget;
  user_group?: string;
}) => {
  const [open, setOpen] = useState(false);
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" className="flex gap-2 items-center">
          Export
          <FileDown className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent className="min-w-[50vw]">
        <DialogHeader>
          <DialogTitle>Export to toml</DialogTitle>
        </DialogHeader>
        <ExportLoader />
      </DialogContent>
    </Dialog>
  );
};

const ExportLoader = () => {
  const { data, isPending } = useRead("ExportAllResourcesToToml", {});
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
    <div className="relative flex justify-center">
      {loading && <Loader2 className="w-8 h-8 animate-spin" />}
      <div className="overflow-y-scroll max-h-[80vh]">
        <pre className="h-fit">{content}</pre>
      </div>
      <CopyButton content={content} className="absolute top-4 right-4" />
    </div>
  );
};
