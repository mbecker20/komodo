import { useRead } from "@hooks";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { Link } from "react-router-dom";
import { Bot, Cloud, Factory } from "lucide-react";

export const BuilderCard = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((builder) => builder._id?.$oid === id);
  if (!builder) return null;

  return (
    <Link to={`/builders/${builder._id?.$oid}`} key={builder._id?.$oid}>
      <Card hoverable>
        <CardHeader className="flex flex-row justify-between">
          <div>
            <CardTitle>{builder.name}</CardTitle>
            <CardDescription> some description</CardDescription>
          </div>
          <Factory className="w-4 h-4" />
        </CardHeader>
        <CardContent className="flex flex-col text-sm text-muted-foreground">
          <div className="flex items-center gap-2">
            <Cloud className="w-4 h-4" />
            AWS
          </div>
          <div className="flex items-center gap-2">
            <Bot className="w-4 h-4" />
            C5x Large
          </div>
        </CardContent>
      </Card>
    </Link>
  );
};
