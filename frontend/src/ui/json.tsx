export const Json = ({ json }: any) => {
  if (!json) {
    return <p>null</p>;
  }

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

  // Type is object or array
  if (Array.isArray(json)) {
    return (
      <div className="flex flex-col gap-2">
        {(json as any[]).map((json) => (
          <Json json={json} />
        ))}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2">
      {Object.keys(json).map((key) => (
        <div className="flex gap-2">
          <p>{key}</p>: <Json json={json[key]} />
        </div>
      ))}
    </div>
  );
};
