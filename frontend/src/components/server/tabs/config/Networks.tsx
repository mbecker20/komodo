import { Component, createSignal, For } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import ConfirmButton from "../../../shared/ConfirmButton";
import Icon from "../../../shared/Icon";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "./Provider";
import { combineClasses } from "../../../../util/helpers";
import { useParams } from "@solidjs/router";

const BASE_NETWORKS = ["bridge", "host", "none"];

const Networks: Component<{}> = (p) => {
  const params = useParams();
  const { networks } = useConfig();
  const filteredNetworks = () => {
    return networks().filter(
      (network) => !BASE_NETWORKS.includes(network.name)
    );
  };
  const [name, setName] = createSignal("");
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <Flex alignItems="center" justifyContent="space-between">
        <h1>networks</h1>
        <Flex alignItems="center">
          <Input
            placeholder="new network name"
            value={name()}
            onEdit={setName}
          />
          <ConfirmButton
            onConfirm={() => {
              if (name().length > 0) {
                // ws.send(CREATE_NETWORK, {
                //   serverID: selected.id(),
                //   name: name(),
                // });
                setName("");
              } else {
                pushNotification("bad", "please enter a name");
              }
            }}
          >
            create
          </ConfirmButton>
        </Flex>
      </Flex>
      {/* <For each={filteredNetworks()}>
        {(network) => <Network network={network} />}
      </For> */}
    </Grid>
  );
};

// const Network: Component<{ network: NetworkType }> = (p) => {
//   const { selected, ws } = useAppState();
//   const { themeClass } = useTheme();
//   return (
//     <Flex
//       class={combineClasses("grey-no-hover", themeClass())}
//       alignItems="center"
//       justifyContent="space-between"
//       style={{
//         padding: "0.5rem",
//       }}
//     >
//       <div>{p.network.name}</div>
//       <ConfirmButton
//         class="red"
//         onConfirm={() => {
//           ws.send(DELETE_NETWORK, {
//             serverID: selected.id(),
//             name: p.network.name,
//           });
//         }}
//       >
//         <Icon type="trash" />
//       </ConfirmButton>
//     </Flex>
//   );
// };

export default Networks;
