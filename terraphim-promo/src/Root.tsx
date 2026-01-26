import "./index.css";
import { Composition } from "remotion";
import { ChatDemo } from "./ChatDemo";
import { Master } from "./Master";
import { LogoCombo } from "./LogoCombo";
import messages from "./messages.json";

export const RemotionRoot: React.FC = () => {
  return (
    <>
      <Composition
        id="ChatDemo"
        component={ChatDemo}
        durationInFrames={30 * (messages.length * 2 + 4)}
        fps={30}
        width={1080}
        height={1920}
        defaultProps={{ messages }}
      />

      {/* Main promo composition (8 seconds) */}
      <Composition
        id="TerraphimSkillsAnnouncement"
        component={Master}
        durationInFrames={240}
        fps={30}
        width={1080}
        height={700}
      />

      {/* Exposed for tweaking */}
      <Composition
        id="LogoCombo"
        component={LogoCombo}
        durationInFrames={240}
        fps={30}
        width={1080}
        height={400}
      />
    </>
  );
};
