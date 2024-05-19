import { Page } from "@components/layouts";
import { ConfirmButton } from "@components/util";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@lib/hooks";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Button } from "@ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { useToast } from "@ui/use-toast";
import { Trash, PlusCircle, Loader2, Check, Tag } from "lucide-react";
import { useState } from "react";
import { Input } from "@ui/input";
import { UpdateUser } from "@components/updates/details";
import { DataTable } from "@ui/data-table";

export const Tags = () => {
  useSetTitle("Tags");

  const [search, setSearch] = useState("");

  const tags = useRead("ListTags", {}).data;

  return (
    <Page
      title="Tags"
      icon={<Tag className="w-8 h-8" />}
      actions={<CreateTag />}
    >
      <div className="flex flex-col gap-4">
        <Input
          placeholder="search..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-[200px] lg:w-[300px]"
        />
        <DataTable
          tableKey="tags"
          data={tags?.filter((tag) => tag.name.includes(search)) ?? []}
          columns={[
            {
              header: "Name",
              accessorKey: "name",
            },
            {
              header: "Owner",
              cell: ({ row }) =>
                row.original.owner ? (
                  <UpdateUser user_id={row.original.owner} />
                ) : (
                  "Unknown"
                ),
            },
            {
              header: "Delete",
              cell: ({ row }) => <DeleteTag tag_id={row.original._id!.$oid} />,
            },
          ]}
        />
      </div>
    </Page>
  );
};

export const TagCards = () => {
  const tags = useRead("ListTags", {}).data;
  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {tags?.map((tag) => (
        <Card
          id={tag._id!.$oid}
          className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors"
        >
          <CardHeader className="flex-row justify-between items-center">
            <CardTitle>{tag.name}</CardTitle>
            <DeleteTag tag_id={tag._id!.$oid} />
          </CardHeader>
          <CardContent className="text-sm text-muted-foreground">
            {tag.owner && (
              <div>
                owner: <UpdateUser user_id={tag.owner} />
              </div>
            )}
          </CardContent>
        </Card>
      ))}
    </div>
  );
};

const CreateTag = () => {
  const { toast } = useToast();
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");
  const invalidate = useInvalidate();
  const { mutate, isPending } = useWrite("CreateTag", {
    onSuccess: () => {
      invalidate(["ListTags"]);
      toast({ title: "Tag Created" });
      setOpen(false);
    },
    onError: (e) => {
      console.log("create tag error:" + e);
      toast({ title: "Failed to create tag" });
      setOpen(false);
    },
  });
  const submit = () => mutate({ name });
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button className="items-center gap-2">
          New Tag <PlusCircle className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Tag</DialogTitle>
        </DialogHeader>
        <div className="py-8 flex flex-col gap-4">
          <div className="flex items-center justify-between">
            Name
            <Input
              className="w-72"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
        </div>
        <DialogFooter className="flex justify-end">
          <Button className="gap-4" onClick={submit} disabled={isPending}>
            Submit
            {isPending ? (
              <Loader2 className="w-4 animate-spin" />
            ) : (
              <Check className="w-4" />
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

const DeleteTag = ({ tag_id }: { tag_id: string }) => {
  const invalidate = useInvalidate();
  const { toast } = useToast();
  const { mutate, isPending } = useWrite("DeleteTag", {
    onSuccess: () => {
      invalidate(["ListTags"]);
      toast({ title: "Tag Deleted" });
    },
    onError: () => {
      toast({ title: "Failed to delete tag" });
    },
  });
  return (
    <ConfirmButton
      title="Delete"
      icon={<Trash className="w-4 h-4" />}
      onClick={() => mutate({ id: tag_id })}
      loading={isPending}
    />
  );
};
