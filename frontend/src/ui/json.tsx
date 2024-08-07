export const Json = ({ json }: any) => {
  const type = typeof json;

  if (type === "function") {
    return <p>??function??</p>;
  }

  // null case
  if (type === "undefined") {
    return <p>null</p>;
  }

  // base cases
  if (
    type === "bigint" ||
    type === "boolean" ||
    type === "number" ||
    type === "string" ||
    type === "symbol"
  ) {
    return <p>{json}</p>;
  }

  // Type is object
  return (
    <div className="flex flex-col gap-2">
      {Object.entries(json).map(([key, json]) => (
        <div className="flex gap-2">
          <p>{key}</p>: <Json json={json} />
        </div>
      ))}
    </div>
  );
};
