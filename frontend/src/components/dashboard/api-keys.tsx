import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { useRead } from "@lib/hooks";
import { Link } from "react-router-dom";
import { Key } from "lucide-react";

export const ApiKeysSummary = () => {
  const keys_count = useRead("ListApiKeys", {}).data?.length;

  return (
    <Link to="/keys" className="w-full">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Api Keys</CardTitle>
              <CardDescription>{keys_count} Total</CardDescription>
            </div>
            <Key className="w-4 h-4" />
          </div>
        </CardHeader>
      </Card>
    </Link>
  );
};
