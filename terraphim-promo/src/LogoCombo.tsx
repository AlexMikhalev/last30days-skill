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

  const scale = interpolate(frame, [0, fps * 0.5], [0.8, 1], {
    extrapolateRight: "clamp",
    easing: Easing.out(Easing.cubic),
  });

  const opacity = interpolate(frame, [0, fps * 0.25], [0, 1], {
    extrapolateRight: "clamp",
  });

  return (
    <AbsoluteFill className="flex items-center justify-center">
      <span
        style={{
          fontFamily: "GT Planar, ui-sans-serif, system-ui, sans-serif",
          fontSize: 60,
          fontWeight: 800,
          color: "#111",
          transform: `scale(${scale})`,
          opacity,
          letterSpacing: -0.5,
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
      <div className="flex items-center gap-10">
        <Img
          src={staticFile("terraphim-logo.png")}
          className="h-24"
          alt="Terraphim"
        />

        <span className="text-6xl font-light text-gray-600">+</span>

        {/* Claude Code: no official redistributable logo in-repo; using a clean wordmark badge */}
        <div className="px-6 py-3 rounded-2xl bg-white/70 border border-black/10 shadow-sm">
          <span className="text-4xl font-semibold text-[#111]">Claude Code</span>
        </div>

        {/* OpenCode logo from anomalyco/opencode repo */}
        <Img
          src={staticFile("opencode-logo.svg")}
          className="h-16"
          alt="OpenCode"
        />
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
