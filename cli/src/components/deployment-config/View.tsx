import React, { Fragment } from "react";
import { Box, Text } from "ink";
import { StartConfig } from "../../types";

const View = ({ url, config }: { url: string, config?: StartConfig }) => {
	const { name, port, volume, restart } = config || { name: "", port: "", volume: "", restart: "" };
	return (
    <Box flexDirection="column" marginLeft={2}>
      <Text color="green">
        url: <Text color="white">{url}</Text>
      </Text>
      {config && (
        <Fragment>
          <Text color="green">
            name: <Text color="white">{name}</Text>
          </Text>
          <Text color="green">
            port: <Text color="white">{port}</Text>
          </Text>
          <Text color="green">
          	mount folder: <Text color="white">{volume || "don't use"}</Text>
          </Text>
          <Text color="green">
            restart: <Text color="white">{restart}</Text>
          </Text>
        </Fragment>
      )}
    </Box>
  );
}

export default View;