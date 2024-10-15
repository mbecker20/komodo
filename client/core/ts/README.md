# Komodo

_A system to build and deploy software across many servers_

```sh
npm install komodo_client
```

or

```sh
yarn add komodo_client
```

```ts
import { KomodoClient, Types } from "komodo_client";

const komodo = KomodoClient("https://demo.komo.do", {
  type: "api-key",
  params: {
    api_key: "your_key",
    secret: "your secret",
  },
});

const stacks: Types.StackListItem[] = await komodo.read({
	type: "ListStacks",
	params: {},
});

const stack: Types.Stack = await komodo.read({
	type: "GetStack",
	params: {
		stack: stacks[0].name,
	}
});
```
