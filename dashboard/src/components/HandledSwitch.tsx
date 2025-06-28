import { useState } from "react";

interface Props {
  isOn: boolean;
  onToggle: (isOn: boolean) => void;
}

export const HandledSwitch = ({ isOn, onToggle }: Props) => {
  return (
    <div
      className="relative w-18 h-18 cursor-pointer"
      onClick={() => onToggle(!isOn)}
    >
      {/* Background Pipe */}
      <div
        className="absolute top-1/2 left-1/2 w-32 h-10 -translate-x-1/2 -translate-y-1/2
                   bg-gradient-to-r from-background via-zinc-400 to-background
                   border-t-2 border-b-2 border-black -z-10"
      />

      {/* Outer Ring */}
      <div
        className={`
          w-full h-full rounded-full border-2 border-black transition-all duration-300 ease-in-out
          ${isOn ? "bg-primary" : "bg-zinc-300"}
        `}
      />

      {/* Inner Circle (creates the ring effect) */}
      <div className="absolute top-1/2 left-1/2 w-10 h-10 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-black bg-zinc-300" />

      {/* Handle */}
      <div
        className={`
          absolute -top-14 left-1/2 w-5 h-16 -translate-x-1/2 
          transition-all duration-300 ease-in-out
          rounded-sm border-2 border-black
          ${isOn ? "bg-primary rotate-90" : "bg-zinc-300 rotate-0"}
        `}
        style={{ transformOrigin: "50% calc(100% + 28px)" }}
      />
    </div>
  );
};
