import {
  AbsoluteFill,
  Img,
  Series,
  staticFile,
  useCurrentFrame,
  useVideoConfig,
  interpolate,
  Easing,
} from "remotion";

const AnnouncementText: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const scale = interpolate(frame, [0, fps * 0.5], [0, 1], {
    extrapolateRight: "clamp",
    easing: Easing.out(Easing.cubic),
  });

  return (
    <AbsoluteFill className="flex items-center justify-center">
      <span
        style={{
          // GT Planar is not bundled here; we keep the requested family name as first choice.
          fontFamily: "GT Planar, ui-sans-serif, system-ui, sans-serif",
          fontSize: 60,
          fontWeight: 700,
          color: "#333",
          transform: `scale(${scale})`,
        }}
      >
        Agent Skills now available
      </span>
    </AbsoluteFill>
  );
};

const Logos: React.FC = () => {
  return (
    <AbsoluteFill className="flex items-center justify-center">
      <div className="flex items-center gap-12">
        <Img
          src={staticFile("terraphim-logo.png")}
          className="h-28"
          alt="Terraphim"
        />
        <span className="text-6xl font-light text-gray-600">+</span>
        <div className="px-6 py-3 rounded-2xl bg-white/70 border border-black/10">
          <span className="text-4xl font-semibold text-[#111]">Claude Code</span>
        </div>
        <div className="px-6 py-3 rounded-2xl bg-white/70 border border-black/10">
          <span className="text-3xl font-semibold text-[#111]">OpenCode</span>
        </div>
      </div>
    </AbsoluteFill>
  );
};

export const LogoCombo: React.FC = () => {
  return (
    <Series>
      {/* 2 seconds */}
      <Series.Sequence durationInFrames={60}>
        <AnnouncementText />
      </Series.Sequence>
      {/* Remaining time */}
      <Series.Sequence durationInFrames={180}>
        <Logos />
      </Series.Sequence>
    </Series>
  );
};
