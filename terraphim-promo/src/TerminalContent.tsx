import { useCurrentFrame, useVideoConfig, interpolate } from "remotion";
import { Cursor } from "./Cursor";

const COMMAND = "npx skills install terraphim/terraphim-skills";

// Taken from actually running the command (ANSI stripped)
const SKILLS_LOGO = `███████╗██╗  ██╗██╗██╗     ██╗     ███████╗
██╔════╝██║ ██╔╝██║██║     ██║     ██╔════╝
███████╗█████╔╝ ██║██║     ██║     ███████╗
╚════██║██╔═██╗ ██║██║     ██║     ╚════██║
███████║██║  ██╗██║███████╗███████╗███████║
╚══════╝╚═╝  ╚═╝╚═╝╚══════╝╚══════╝╚══════╝`;

const OUTPUT_LINES = [
  "",
  "┌   skills",
  "│",
  "│  Tip: use the --yes (-y) and --global (-g) flags to install without prompts.",
  "│",
  "◇  Source: https://github.com/terraphim/terraphim-skills.git",
  "│",
  "◒  Cloning repository",
  "◇  Repository cloned",
  "│",
  "◇  Found 31 skills",
  "│",
  "◆  Select skills to install (space to toggle)",
  "│",
  "└",
];

export const TerminalContent: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const charsPerSecond = 18;
  const framesPerChar = fps / charsPerSecond;

  const typingEndFrame = COMMAND.length * framesPerChar;
  const outputStartFrame = typingEndFrame + fps * 0.5; // 0.5s delay

  const visibleChars = Math.floor(
    interpolate(frame, [0, typingEndFrame], [0, COMMAND.length], {
      extrapolateRight: "clamp",
    })
  );

  const displayedText = COMMAND.slice(0, visibleChars);
  const isTyping = visibleChars < COMMAND.length;

  const showOutput = frame >= outputStartFrame;

  // Stagger: one line every 50ms
  const framesPerLine = fps * 0.05;
  const linesStartFrame = outputStartFrame + framesPerLine;

  const visibleLines = Math.floor(
    interpolate(
      frame,
      [linesStartFrame, linesStartFrame + OUTPUT_LINES.length * framesPerLine],
      [0, OUTPUT_LINES.length],
      { extrapolateLeft: "clamp", extrapolateRight: "clamp" }
    )
  );

  return (
    <div className="flex-1 bg-white p-8 font-mono text-4xl overflow-hidden">
      <div className="flex items-center text-[#333]">
        <span className="text-[#2ecc71] font-semibold">~</span>
        <span className="mx-3">$</span>
        <span style={{ whiteSpace: "pre-wrap" }}>{displayedText}</span>
        {!showOutput && <Cursor blinking={!isTyping} />}
      </div>

      {showOutput && (
        <div className="mt-6 text-lg leading-tight">
          <pre className="text-[#333]">{SKILLS_LOGO}</pre>
          {OUTPUT_LINES.slice(0, visibleLines).map((line, i) => (
            <div key={i} className="text-[#333]" style={{ whiteSpace: "pre-wrap" }}>
              {line}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
