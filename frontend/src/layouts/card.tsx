// import {
//   Accordion,
//   AccordionContent,
//   AccordionItem,
//   AccordionTrigger,
// } from "@ui/accordion";
// import { Badge } from "@ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { ReactNode } from "react";

interface CardProps {
  title: string;
  description: string;
  // icon: ReactNode;
  children: ReactNode;
  statusIcon?: ReactNode;
}

export const ResourceCard = ({
  title,
  description,
  // icon,
  children,
  statusIcon,
}: CardProps) => (
  <Card hoverable>
    <CardHeader className="flex flex-row justify-between">
      <div>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </div>
      <div className="flex gap-2">
        {statusIcon}
        {/* {icon} */}
      </div>
    </CardHeader>
    <CardContent className="flex flex-col gap-6">
      {/* {icon}
      <div className="border h-6" /> */}
      {children}
      {/* <Accordion
        type="single"
        collapsible
        onClick={(e) => {
          e.stopPropagation();
          e.preventDefault();
        }}
      >
        <AccordionItem value="tags">
          <AccordionTrigger>Show Tags</AccordionTrigger>
          <AccordionContent>
            <div className="flex gap-2 flex-wrap">
              <Badge>crawler</Badge>
              <Badge>prod</Badge>
              <Badge>cex</Badge>
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion> */}
    </CardContent>
  </Card>
);
