const getFlags = async () => {
  const meow = await import("meow");
  const cli = meow.default(
    `
			Usage
				$ npx @mbecker20/monitor-cli
			Options
				--core, -c  setup monitor core
				--periphery, -p  setup monitor periphery
        --restart-default -r  restart monitor with defaults
			Examples
				$ npx @mbecker20/monitor-cli --core
		`,
    {
      importMeta: import.meta,
      flags: {
        core: {
          type: "boolean",
          alias: "-c",
        },
        periphery: {
          type: "boolean",
          alias: "-p",
        },
        restartDefault: {
          type: "boolean",
          alias: "-r",
        }
      },
    }
  );
  return cli.flags
};

export default getFlags;
