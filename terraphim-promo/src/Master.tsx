import {
  AbsoluteFill,
  Sequence,
  useCurrentFrame,
  useVideoConfig,
  interpolate,
  spring,
} from "remotion";
import { MacTerminal } from "./MacTerminal";
import { LogoCombo } from "./LogoCombo";
import { getOutputDoneFrame } from "./TerminalContent";

export const Master: React.FC = () => {
  const frame = useCurrentFrame();
  const { durationInFrames, fps } = useVideoConfig();

  const outputDoneFrame = Math.min(durationInFrames - 1, getOutputDoneFrame(fps));

  // Fast slide-in from bottom, no bounce
  const slideIn = spring({
    frame,
    fps,
    config: {
      damping: 200,
      stiffness: 100,
    },
  });

  const translateY = interpolate(slideIn, [0, 1], [700, 0]);

  // Subtle parallax rotation
  const rotateY = interpolate(frame, [0, durationInFrames], [10, -10]);

  // Gentle zoom
  const scale = interpolate(frame, [0, durationInFrames], [0.9, 1]);

  // After output is done, flip towards camera by rotating around bottom edge
  const flipOut = spring({
    frame: frame - outputDoneFrame,
    fps,
    config: {
      damping: 200,
      stiffness: 100,
    },
  });

  const flipRotateX = frame >= outputDoneFrame
    ? interpolate(flipOut, [0, 1], [0, -70])
    : 0;

  // The 3D transform can leave tiny raster artifacts at the edges.
  // Fade the terminal out shortly after the flip starts, and stop rendering it afterwards.
  const fadeStart = outputDoneFrame + fps * 0.35;
  const fadeEnd = outputDoneFrame + fps * 0.55;
  const terminalOpacity = frame < fadeStart
    ? 1
    : interpolate(frame, [fadeStart, fadeEnd], [1, 0], {
        extrapolateLeft: "clamp",
        extrapolateRight: "clamp",
      });

  const terminalSequenceDuration = Math.min(
    durationInFrames,
    Math.ceil(fadeEnd + fps * 0.05)
  );

  return (
    <AbsoluteFill
      className="bg-[#f8fafc]"
      style={{ perspective: 1000, overflow: "hidden" }}
    >
      {/* LogoCombo behind the terminal */}
      <Sequence from={outputDoneFrame} durationInFrames={durationInFrames - outputDoneFrame}>
        <LogoCombo />
      </Sequence>

      {/* Terminal on top */}
      <Sequence
        durationInFrames={terminalSequenceDuration}
        style={{
          transform: `translateY(${translateY}px) rotateX(${flipRotateX}deg) rotateY(${rotateY}deg) scale(${scale})`,
          transformOrigin: "50% 100%",
          opacity: terminalOpacity,
        }}
      >
        <MacTerminal />
      </Sequence>

      {/* Safety matte to hide any bottom-edge raster artifacts from 3D transforms */}
      <AbsoluteFill
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          bottom: 0,
          height: 320,
          backgroundColor: "#f8fafc",
          zIndex: 9999,
        }}
      />
    </AbsoluteFill>
  );
};
