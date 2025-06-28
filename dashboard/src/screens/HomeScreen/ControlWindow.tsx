import { HandledSwitch } from "@/components/HandledSwitch";
import { useState } from "react";

interface ControlItemProps {
  title: string;
  isOn: boolean;
  onToggle: (isOn: boolean) => void;
}

const ControlItem = ({ title, isOn, onToggle }: ControlItemProps) => (
  <div className="w-fit flex flex-col items-center gap-2">
    <HandledSwitch isOn={isOn} onToggle={onToggle} />
    <span className="text-xl font-bold text-center">{title}</span>
  </div>
);

export const ControlWindow = () => {
  const [isZone1On, setIsZone1On] = useState(false);
  const [isZone2On, setIsZone2On] = useState(false);
  const [isZone3On, setIsZone3On] = useState(false);
  const [isZone4On, setIsZone4On] = useState(false);
  const [isZone5On, setIsZone5On] = useState(false);
  const [isZone6On, setIsZone6On] = useState(false);

  const handleToggleZone = (zone: number) => {
    switch (zone) {
      case 1:
        setIsZone1On(!isZone1On);
        break;
      case 2:
        setIsZone2On(!isZone2On);
        break;
      case 3:
        setIsZone3On(!isZone3On);
        break;
      case 4:
        setIsZone4On(!isZone4On);
        break;
      case 5:
        setIsZone5On(!isZone5On);
        break;
      case 6:
        setIsZone6On(!isZone6On);
        break;
    }
  };

  return (
    <div className="w-full flex-1 flex flex-col justify-center items-center px-10">
      <div className="w-full grid grid-cols-3 gap-x-4 gap-y-24 justify-items-center">
        <ControlItem
          title="Zone 1"
          isOn={isZone1On}
          onToggle={() => handleToggleZone(1)}
        />
        <ControlItem
          title="Zone 2"
          isOn={isZone2On}
          onToggle={() => handleToggleZone(2)}
        />
        <ControlItem
          title="Zone 3"
          isOn={isZone3On}
          onToggle={() => handleToggleZone(3)}
        />
        <ControlItem
          title="Zone 4"
          isOn={isZone4On}
          onToggle={() => handleToggleZone(4)}
        />
        <ControlItem
          title="Zone 5"
          isOn={isZone5On}
          onToggle={() => handleToggleZone(5)}
        />
        <ControlItem
          title="Zone 6"
          isOn={isZone6On}
          onToggle={() => handleToggleZone(6)}
        />
      </div>
    </div>
  );
};
