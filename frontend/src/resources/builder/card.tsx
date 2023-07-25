import { useRead } from "@hooks";
import { ServerStatusIcon } from "@resources/server/util";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { Link } from "react-router-dom";
import { Factory } from "lucide-react";

export const BuilderCard = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((builder) => builder._id?.$oid === id);
  if (!builder) return null;

  return (
    <Link to={`/builds/${builder._id?.$oid}`} key={builder._id?.$oid}>
      <Card hoverable>
        <CardHeader className="flex flex-row justify-between">
          <div>
            <CardTitle>{builder.name}</CardTitle>
            <CardDescription> </CardDescription>
          </div>
          <ServerStatusIcon serverId={builder._id?.$oid} />
        </CardHeader>
        <CardContent className="flex items-center  gap-4">
          <Factory className="w-4 h-4" />
          <div className="border h-6" />
          <div>{builder.description}</div>
        </CardContent>
      </Card>
    </Link>
  );
};
