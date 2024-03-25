import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { useRead } from "@lib/hooks";
import { Link } from "react-router-dom";
import { Tag } from "lucide-react";

export const TagsSummary = () => {
  const tags_count = useRead("ListTags", {}).data?.length;

  return (
    <Link to="/tags" className="w-full">
      <Card>
        <CardHeader className="justify-between">
          <div>
            <CardTitle>Tags</CardTitle>
            <CardDescription>{tags_count} Total</CardDescription>
          </div>
          <Tag className="w-4 h-4" />
        </CardHeader>
      </Card>
    </Link>
  );
};
