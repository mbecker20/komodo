import { useRead } from "@hooks";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { Link } from "react-router-dom";

export const DeploymentCard = ({ id }: { id: string }) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === id);
  if (!deployment) return null;
  return (
    <Link to={`/deployments/${deployment.id}`}>
      <Card hoverable>
        <CardHeader>
          <CardTitle>{deployment.name}</CardTitle>
          <CardDescription>
            {deployment.status ?? "not deployed"}
          </CardDescription>
        </CardHeader>
      </Card>
    </Link>
  );
};
