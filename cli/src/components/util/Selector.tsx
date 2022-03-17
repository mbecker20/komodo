import React, { useState } from "react";
import { Box, Text, useInput } from "ink";

const Selector = (p: {
	items: string[];
	onSelect?: (item: string, i: number) => void;
}) => {
	const [highlighted, setHighlighted] = useState(0);
	useInput((_, key) => {
		if (key.upArrow) {
			setHighlighted(Math.max(highlighted - 1, 0));
		} else if (key.downArrow) {
			setHighlighted(Math.min(highlighted + 1, p.items.length - 1));
		} else if (key.return) {
			if (p.onSelect) p.onSelect(p.items[highlighted]!, highlighted);
		}
	});
	return (
		<Box flexDirection="column">
			{p.items.map((item, i) => {
				return (
          <Text key={i} color={i === highlighted ? "green" : "white"}>
            {i === highlighted ? ">" : " "} {item}
          </Text>
        );
			})}
		</Box>
	);
};

export default Selector;
