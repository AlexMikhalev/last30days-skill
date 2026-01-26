import { AbsoluteFill } from "remotion";
import { TerminalContent } from "./TerminalContent";

export const MacTerminal: React.FC = () => {
  return (
    // Transparent background + margin (as in JonnyBurger prompt)
    <AbsoluteFill className="p-10" style={{ overflow: "hidden" }}>
      <div
        className="w-full h-full flex flex-col rounded-2xl overflow-hidden"
        style={{
          // Avoid extreme shadows that can create edge artifacts when 3D-transformed
          boxShadow: "0 20px 40px -20px rgba(0,0,0,0.35)",
          backfaceVisibility: "hidden",
          transform: "translateZ(0)",
        }}
      >
        {/* Title bar */}
        <div className="h-16 bg-[#f6f6f6] flex items-center px-6 border-b border-[#e0e0e0]">
          {/* Traffic lights */}
          <div className="flex gap-3">
            <div className="w-4 h-4 rounded-full bg-[#ff5f57]" />
            <div className="w-4 h-4 rounded-full bg-[#febc2e]" />
            <div className="w-4 h-4 rounded-full bg-[#28c840]" />
          </div>

          {/* Title */}
          <div className="flex-1 text-center">
            <span className="text-[#4d4d4d] text-lg font-medium">Terminal</span>
          </div>

          {/* Spacer for symmetry */}
          <div className="w-16" />
        </div>

        <TerminalContent />
      </div>
    </AbsoluteFill>
  );
};
