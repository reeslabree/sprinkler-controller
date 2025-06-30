import { HandledSwitch } from "@/components";
import { useWebSocket } from "@/contexts";
import { cn } from "@/lib/utils";
import { ToggleZonePayload, ToggleZoneResponse } from "@/types";
import { useEffect, useState } from "react";

interface ControlItemProps {
  title: string;
  isOn: boolean;
  onToggle: (isOn: boolean) => void;
  isDisabled?: boolean;
}

const ControlItem = ({
  title,
  isOn,
  onToggle,
  isDisabled,
}: ControlItemProps) => (
  <div className="w-fit flex flex-col items-center gap-2">
    <HandledSwitch isOn={isOn} onToggle={onToggle} isDisabled={isDisabled} />
    <span
      className={cn(
        "text-xl font-bold text-center",
        isDisabled && "text-zinc-400"
      )}
    >
      {title}
    </span>
  </div>
);

export const ControlWindow = () => {
  const { sendMessage, isClientConnected, isControllerConnected } =
    useWebSocket();
  const isConnected = isClientConnected && isControllerConnected;

  const [isZone1On, setIsZone1On] = useState(false);
  const [isZone2On, setIsZone2On] = useState(false);
  const [isZone3On, setIsZone3On] = useState(false);
  const [isZone4On, setIsZone4On] = useState(false);
  const [isZone5On, setIsZone5On] = useState(false);
  const [isZone6On, setIsZone6On] = useState(false);

  const handleToggleZone = (zone: number) => {
    let isOn = false;

    switch (zone) {
      case 1:
        isOn = !isZone1On;
        setIsZone1On(isOn);
        break;
      case 2:
        isOn = !isZone2On;
        setIsZone2On(isOn);
        break;
      case 3:
        isOn = !isZone3On;
        setIsZone3On(isOn);
        break;
      case 4:
        isOn = !isZone4On;
        setIsZone4On(isOn);
        break;
      case 5:
        isOn = !isZone5On;
        setIsZone5On(isOn);
        break;
      case 6:
        isOn = !isZone6On;
        setIsZone6On(isOn);
        break;
    }

    sendMessage<ToggleZonePayload, ToggleZoneResponse>({
      type: "toggleZone",
      payload: { zone, activate: isOn },
    });
  };

  return (
    <div className="w-full flex-1 flex flex-col justify-center items-center px-10">
      <div className="w-full grid grid-cols-3 gap-x-4 gap-y-24 justify-items-center">
        <ControlItem
          title="Zone 1"
          isOn={isZone1On}
          onToggle={() => handleToggleZone(1)}
          isDisabled={!isConnected}
        />
        <ControlItem
          title="Zone 2"
          isOn={isZone2On}
          onToggle={() => handleToggleZone(2)}
          isDisabled={!isConnected}
        />
        <ControlItem
          title="Zone 3"
          isOn={isZone3On}
          onToggle={() => handleToggleZone(3)}
          isDisabled={!isConnected}
        />
        <ControlItem
          title="Zone 4"
          isOn={isZone4On}
          onToggle={() => handleToggleZone(4)}
          isDisabled={!isConnected}
        />
        <ControlItem
          title="Zone 5"
          isOn={isZone5On}
          onToggle={() => handleToggleZone(5)}
          isDisabled={!isConnected}
        />
        <ControlItem
          title="Zone 6"
          isOn={isZone6On}
          onToggle={() => handleToggleZone(6)}
          isDisabled={!isConnected}
        />
      </div>
    </div>
  );
};
