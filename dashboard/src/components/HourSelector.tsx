import { ActivePeriod, Zone } from "@/types";
import { useState, useRef, useEffect } from "react";

interface Props {
  zone: Zone;
  setActivePeriod: (activePeriod: ActivePeriod) => any;
  activePeriod?: ActivePeriod;
}

export const HourSelector = ({
  zone,
  activePeriod,
  setActivePeriod,
}: Props) => {
  const [durationMinutes, setDurationMinutes] = useState(
    activePeriod?.durationMinutes || 0
  );
  const [isDragging, setIsDragging] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const isInitialMount = useRef(true);

  useEffect(() => {
    setDurationMinutes(activePeriod?.durationMinutes || 0);
  }, [activePeriod]);

  const handleSetActivePeriod = () => {
    setActivePeriod({
      zone,
      durationMinutes,
    });
  };

  useEffect(() => {
    if (isInitialMount.current) {
      isInitialMount.current = false;
      return;
    }

    const currentDuration = activePeriod?.durationMinutes || 0;
    if (durationMinutes !== currentDuration) {
      handleSetActivePeriod();
    }
  }, [durationMinutes]);

  const getAngleFromMouse = (e: React.MouseEvent | MouseEvent) => {
    if (!containerRef.current) return 0;
    const rect = containerRef.current.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;
    const mouseX = e.clientX;
    const mouseY = e.clientY;

    const angle =
      (Math.atan2(mouseY - centerY, mouseX - centerX) * 180) / Math.PI;
    return (angle + 90 + 360) % 360;
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    const angle = getAngleFromMouse(e);
    const minutes = Math.round((angle / 360) * 60);
    setDurationMinutes(minutes);
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (!isDragging) return;
    const angle = getAngleFromMouse(e);
    const minutes = Math.round((angle / 360) * 60);
    setDurationMinutes(Math.max(0, minutes));
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  useEffect(() => {
    if (isDragging) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
      return () => {
        document.removeEventListener("mousemove", handleMouseMove);
        document.removeEventListener("mouseup", handleMouseUp);
      };
    }
  }, [isDragging]);

  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="relative w-48 h-48">
        <div
          ref={containerRef}
          className="w-48 h-48 rounded-full bg-zinc-200 border-2 border-black flex items-center justify-center relative"
          onMouseDown={handleMouseDown}
        >
          <div
            className="absolute inset-0 rounded-full cursor-pointer"
            style={{ pointerEvents: "none" }}
          />
          <div
            className="absolute rounded-full cursor-pointer"
            style={{
              width: "192px",
              height: "192px",
              left: "0px",
              top: "0px",

              pointerEvents: "auto",
            }}
            onMouseDown={handleMouseDown}
          />
          {durationMinutes > 0 && (
            <div
              className="absolute inset-0 rounded-full cursor-pointer"
              style={{
                background: `conic-gradient(from 0deg, #22c55e 0deg, #22c55e ${
                  (durationMinutes / 60) * 360
                }deg, transparent ${(durationMinutes / 60) * 360}deg)`,
                zIndex: 0,
              }}
            />
          )}

          <div className="w-40 h-40 rounded-full bg-white border-2 border-black flex items-center justify-center relative z-10">
            {Array.from({ length: 12 }, (_, i) => {
              const angle = i * 30 - 91.5;
              const radius = 75;
              const x = Math.cos((angle * Math.PI) / 180) * radius;
              const y = Math.sin((angle * Math.PI) / 180) * radius;

              return (
                <div
                  key={i}
                  className="absolute w-1 h-3 bg-black rounded-b-sm"
                  style={{
                    left: `${80 + x}px`,
                    top: `${80 + y}px`,
                    transform: `translate(-50%, -50%) rotate(${
                      angle + 91.5
                    }deg)`,
                    transformOrigin: "0px 4px",
                  }}
                />
              );
            })}
            {Array.from({ length: 60 }, (_, i) => {
              const angle = i * 6 - 91.5;
              const radius = 75;
              const x = Math.cos((angle * Math.PI) / 180) * radius;
              const y = Math.sin((angle * Math.PI) / 180) * radius;

              return (
                <div
                  key={i}
                  className="absolute w-0.5 h-1.5 bg-black rounded-b-sm"
                  style={{
                    left: `${80 + x}px`,
                    top: `${80 + y}px`,
                    transform: `translate(-50%, -50%) rotate(${
                      angle + 91.5
                    }deg)`,
                    transformOrigin: "-1px 1.3px",
                  }}
                />
              );
            })}
            <div className="text-center select-none">
              <div className="text-sm font-bold text-zinc-800">
                Zone {zone.replace("zone", "")}
              </div>
              <div className="text-lg font-bold text-zinc-900">
                {Math.floor(durationMinutes)}m
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
