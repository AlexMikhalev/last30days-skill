import { useCurrentFrame, useVideoConfig } from "remotion";

export const Cursor: React.FC<{ blinking: boolean }> = ({ blinking }) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  // blink at ~2Hz
  const period = Math.max(1, Math.floor(fps / 2));
  const on = !blinking || Math.floor(frame / period) % 2 === 0;

  return (
    <span
      className="inline-block align-middle ml-1"
      style={{
        width: 10,
        height: 26,
        backgroundColor: "#333",
        opacity: on ? 1 : 0,
      }}
    />
  );
};
