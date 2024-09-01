import React from "react";

export default function KomodoLogo({ width = "4rem" }) {
  return (
    <img
      style={{ width, height: "auto", opacity: 0.7 }}
      src="img/monitor-lizard.png"
      alt="monitor-lizard"
    />
  );
}
