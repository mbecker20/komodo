import { OpenAlerts } from "@components/alert";
import { Page } from "@components/layouts";
import { AllUpdates } from "@components/updates/resource";

export const Dashboard = () => {
	return (
    <Page title="">
      <OpenAlerts />
      <AllUpdates />
    </Page>
  );
}