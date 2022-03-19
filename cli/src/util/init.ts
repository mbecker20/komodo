import getFlags from "./flags";
import { isDockerInstalled } from "./helpers/docker";

// used to load async prerequisites

async function init() {
	const flags = await getFlags();
	const dockerInstalled = await isDockerInstalled();
	return {
		flags,
		dockerInstalled
	}
}

export default init;