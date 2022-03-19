const getFlags = async () => {
  const meow = await import("meow");
  const cli = meow.default(
    `
			Usage
				$ @mbecker20/monitor-cli
			Options
				--core, -c  setup monitor core
				--periphery, -p  setup monitor periphery
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
      },
    }
  );
  return cli.flags
};

export default getFlags;
