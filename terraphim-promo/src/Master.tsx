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

// Roughly when the terminal output has finished appearing.
// (We keep it simple + deterministic like the reference prompt.)
const OUTPUT_DONE_FRAME = 120;

export const Master: React.FC = () => {
  const frame = useCurrentFrame();
  const { durationInFrames, fps } = useVideoConfig();

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

  // After output is "done", flip towards camera by rotating around bottom edge
  const flipOut = spring({
    frame: frame - OUTPUT_DONE_FRAME,
    fps,
    config: {
      damping: 200,
      stiffness: 100,
    },
  });

  const flipRotateX = frame >= OUTPUT_DONE_FRAME
    ? interpolate(flipOut, [0, 1], [0, -70])
    : 0;

  return (
    <AbsoluteFill className="bg-[#f8fafc]" style={{ perspective: 1000 }}>
      {/* LogoCombo behind the terminal */}
      <Sequence
        from={OUTPUT_DONE_FRAME}
        durationInFrames={durationInFrames - OUTPUT_DONE_FRAME}
      >
        <LogoCombo />
      </Sequence>

      {/* Terminal on top */}
      <Sequence
        durationInFrames={durationInFrames}
        style={{
          transform: `translateY(${translateY}px) rotateX(${flipRotateX}deg) rotateY(${rotateY}deg) scale(${scale})`,
          transformOrigin: "50% 100%",
        }}
      >
        <MacTerminal />
      </Sequence>
    </AbsoluteFill>
  );
};
