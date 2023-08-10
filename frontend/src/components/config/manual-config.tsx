import { Button } from "@ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { useState } from "react";

interface Layout {
  [tab_name: string]: {
    components: Array<{
      title: string;
      element: React.JSX.Element;
      description?: string;
    }>;
    description?: string;
  };
}

export const ManualConfig = <L extends Layout>({ layout }: { layout: L }) => {
  const tab_names = Object.keys(layout);
  const [show, setShow] = useState(tab_names[0]);

  return (
    <div className="flex gap-4">
      <div className="flex flex-col gap-4 w-[300px]">
        {tab_names.map((key) => (
          <Button
            key={key}
            onClick={() => setShow(key)}
            variant={key === show ? "secondary" : "outline"}
            className="capitalize justify-start"
          >
            {key}
          </Button>
        ))}
      </div>
      <Card className="w-full">
        <CardHeader>
          <CardTitle className="capitalize"> {show} </CardTitle>
          <CardDescription> {layout[show].description} </CardDescription>
        </CardHeader>
        <CardContent>
          {layout[show].components.map((item) => (
            <div
              className="flex justify-between items-center border-b pb-4"
              key={item.title}
            >
              <div className="capitalize"> {item.title} </div>
              {item.element}
            </div>
          ))}
        </CardContent>
      </Card>
    </div>
  );
};
