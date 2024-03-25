import { Page, Section } from "@components/layouts";
import { ConfirmButton } from "@components/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { fmt_date } from "@lib/utils";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { useToast } from "@ui/use-toast";
import { Key, Trash } from "lucide-react";

export const Keys = () => {
  return (
    <Page title="">
      <ApiKeysList />
    </Page>
  );
};

export const ApiKeysList = () => {
  const keys = useRead("ListApiKeys", {}).data;
  return (
    <Section title="Api Keys" icon={<Key className="w-4 h-4" />}>
      <div className="flex flex-col lg:flex-row gap-4 w-full">
        {keys?.map((key) => (
          <Card
            id={key.key}
            className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors"
          >
            <CardHeader className="flex-row justify-between items-center">
              <CardTitle>{key.name}</CardTitle>
              <DeleteKey api_key={key.key} />
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">
              <div>created at: {fmt_date(new Date(key.created_at))}</div>
              <div>
                expires:{" "}
                {key.expires === 0 ? "never" : fmt_date(new Date(key.expires))}
              </div>
              <div>{key.key}</div>
            </CardContent>
          </Card>
        ))}
      </div>
    </Section>
  );
};

const DeleteKey = ({ api_key }: { api_key: string }) => {
  const invalidate = useInvalidate();
  const { toast } = useToast();
  const { mutate, isPending } = useWrite("DeleteApiKey", {
    onSuccess: () => {
      invalidate(["ListApiKeys"]);
      toast({ title: "Api Key Deleted" });
    },
    onError: () => {
      toast({ title: "Failed to delte api key",  })
    }
  });
  return (
    <ConfirmButton
      title="Delete"
      icon={<Trash className="w-4 h-4" />}
      onClick={() => mutate({ key: api_key })}
      loading={isPending}
    />
  );
};
