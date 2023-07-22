import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { ReactNode } from "react";

export const Resources = ({
  title,
  newButton,
  children,
}: {
  title: string;
  newButton: ReactNode;
  children: ReactNode;
}) => {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between">
        <CardTitle className="text-3xl">{title}</CardTitle>
        {newButton}
      </CardHeader>
      <CardContent className="h-fit min-h-[50vh] max-h-[70vh] overflow-auto">
        <div className="grid gap-4 md:grid-cols-2">{children}</div>
      </CardContent>
    </Card>
  );
};
