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
  icon: ReactNode;
  children: ReactNode;
  statusIcon?: ReactNode;
}

export const ResourceCard = ({
  title,
  description,
  icon,
  children,
  statusIcon,
}: CardProps) => (
  <Card hoverable>
    <CardHeader className="flex flex-row justify-between">
      <div>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </div>
      {statusIcon}
    </CardHeader>
    <CardContent className="flex items-center gap-4">
      {icon}
      <div className="border h-6" />
      {children}
    </CardContent>
  </Card>
);
