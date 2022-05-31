import { Component, createSignal, Show } from "solid-js";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import { useToggle } from "../../../../../util/hooks";
import { postDotEnv } from "../../../../../util/query";
import Button from "../../../../util/Button";
import ConfirmButton from "../../../../util/ConfirmButton";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import CenterMenu from "../../../../util/menu/CenterMenu";
import TextArea from "../../../../util/TextArea";
import { useConfig } from "../Provider";

const AddDotEnv: Component<{}> = (p) => {
  const [show, toggle] = useToggle();
	const [dotenv, setDotEnv] = createSignal("");
	const toggleShow = () => {
    setDotEnv("");
		toggle()
  };
	const { deployment } = useConfig();
  const save = async () => {
    await postDotEnv(deployment._id!, dotenv());
		toggleShow();
  };
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target={<Button class="blue">add dotenv</Button>}
      content={
        <Grid gap="1rem">
          <Flex alignItems="center" justifyContent="space-between">
            <h1>{deployment.name} environment</h1>
            <Show when={dotenv().length > 0}>
              <ConfirmButton color="green" onConfirm={save}>
                save
              </ConfirmButton>
            </Show>
          </Flex>
          <TextArea
            class="scroller"
            value={dotenv()}
            onEdit={setDotEnv}
            style={{ width: "40rem", "max-width": "90vw", height: "80vh" }}
          />
        </Grid>
      }
    />
  );
};

export default AddDotEnv;
