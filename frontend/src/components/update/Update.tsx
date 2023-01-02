import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { Update as UpdateType } from "../../types";
import {
  combineClasses,
  readableMonitorTimestamp,
} from "../../util/helpers";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import s from "./update.module.scss";
import UpdateMenu from "./UpdateMenu";

const Update: Component<{ update: UpdateType }> = (p) => {
  const { usernames } = useAppState();
  const operation = () => {
    return p.update.operation.replaceAll("_", " ")
  };
  return (
    <Grid gap="0.25rem" class={combineClasses(s.Update, "shadow")}>
      <div
        style={{
          color: !p.update.success ? "rgb(182, 47, 52)" : "inherit",
        }}
      >
        {operation()}
      </div>
      <div style={{ "place-self": "start end" }}>{readableMonitorTimestamp(p.update.start_ts)}</div>
      <Flex gap="0.5rem" alignItems="center">
        <Icon type="user" />
        <div>{usernames.get(p.update.operator)}</div>
      </Flex>
      <Flex style={{ "place-self": "center end" }} alignItems="center">
        <UpdateMenu update={p.update} />
      </Flex>
    </Grid>
  );
};

export default Update;