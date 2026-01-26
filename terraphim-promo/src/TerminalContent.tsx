import { useCurrentFrame, useVideoConfig } from "remotion";
import { Cursor } from "./Cursor";

const lines = [
  // Remotion prompt adapted for Terraphim skills
  "claude plugin marketplace add terraphim/terraphim-skills",
  "claude plugin install terraphim-engineering-skills@terraphim-skills",
];

export const TerminalContent: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const charsPerSecond = 18;
  const framesPerChar = fps / charsPerSecond;

  const fullText = lines.join("\n");
  const charsToShow = Math.min(fullText.length, Math.floor(frame / framesPerChar));

  const typed = fullText.slice(0, charsToShow);
  const isTyping = charsToShow < fullText.length;

  return (
    <div className="flex-1 bg-white p-8 font-mono text-[34px] leading-[48px]">
      <div className="text-[#333]">
        <span className="text-[#2ecc71] font-semibold">~</span>
        <span className="mx-3">$</span>
        <span style={{ whiteSpace: "pre-wrap" }}>{typed}</span>
        <Cursor blinking={!isTyping} />
      </div>
    </div>
  );
};
