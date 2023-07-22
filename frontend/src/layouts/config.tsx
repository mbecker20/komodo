import { Button } from "@ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { RefreshCcw, Save } from "lucide-react";
import { ReactNode } from "react";

export const Config = ({
  children,
  loading,
  save,
  reset,
}: {
  children: ReactNode;
  loading: boolean;
  save: { disabled?: boolean; onClick: () => void };
  reset: { disabled?: boolean; onClick: () => void };
}) => (
  <Card>
    <CardHeader className="flex-row justify-between">
      <CardTitle className="text-xl"> Config </CardTitle>
      <div className="flex gap-4">
        <Button
          variant="outline"
          disabled={loading || reset.disabled}
          intent="warning"
          className="flex gap-2"
          onClick={reset.onClick}
        >
          Reset <RefreshCcw className="h-4 w-4" />
        </Button>
        <Button
          variant="outline"
          disabled={loading || save.disabled}
          intent="success"
          className="flex gap-2"
          onClick={save.onClick}
        >
          {loading ? "Saving..." : "Save"}
          <Save className="h-4 w-4" />
        </Button>
      </div>
    </CardHeader>
    <CardContent className="flex flex-col gap-4 max-h-[50vh] overflow-y-auto">
      {children}
    </CardContent>
  </Card>
);
