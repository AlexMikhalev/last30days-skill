import "./index.css";
import { Composition } from "remotion";
import { ChatDemo } from "./ChatDemo";
import { MacTerminal } from "./MacTerminal";
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

      <Composition
        id="TerraphimSkillsInstall"
        component={MacTerminal}
        durationInFrames={300}
        fps={30}
        width={1280}
        height={1000}
      />
    </>
  );
};
