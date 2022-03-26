import { Component, createEffect, createSignal } from "solid-js";
import { pushNotification } from "../..";
import { CREATE_DEPLOYMENT } from "../../state/actions";
import { defaultDeployment } from "../../state/defaults";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import Icon from "../util/icons/Icon";
import Input from "../util/Input";
import Flex from "../util/layout/Flex";
import CenterMenu from "../util/menu/CenterMenu";

const CreateDeployment: Component<{ serverID: string }> = (p) => {
  const { username } = useUser();
  const { servers, ws } = useAppState();
  const server = () => servers.get(p.serverID);
  const [name, setName] = createSignal("");
  return (
    <CenterMenu
      title={`create deployment on ${server()?.name}`}
      target={<Icon type="plus" />}
      targetClass="green"
      targetStyle={{ width: "100%" }}
      content={
        <>
          <Flex alignItems="center" justifyContent="space-between">
            <Input
              value={name()}
							onEdit={setName}
              placeholder="name"
              autofocus
              style={{ "font-size": "1.5rem" }}
            />
            <button
              class="green"
              style={{ width: "100%" }}
              onClick={() => {
                if (name().length > 0) {
									ws.send(CREATE_DEPLOYMENT, {
                    deployment: defaultDeployment(
                      name(),
                      p.serverID,
                      username()!
                    ),
                  });
									pushNotification("ok", "creating deployment...");
								}
              }}
            >
              create
            </button>
          </Flex>
        </>
      }
    />
  );
};

export default CreateDeployment;
