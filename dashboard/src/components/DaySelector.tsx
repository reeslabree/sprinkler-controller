import { cn } from "@/lib/utils";
import { Day } from "@/types/schedules";

interface Props {
  selectedDays: Day[];
  setSelectedDays: (days: Day[]) => void;
  className?: string;
}

export const DaySelector = ({
  selectedDays,
  setSelectedDays,
  className,
}: Props) => {
  const toggleDay = (day: Day) => {
    if (selectedDays.includes(day)) {
      setSelectedDays(selectedDays.filter((d) => d !== day));
    } else {
      setSelectedDays([...selectedDays, day]);
    }
  };

  const getDayDisplay = (day: Day) => {
    return day.charAt(0).toUpperCase() + day.slice(1, 3);
  };

  return (
    <div className={cn("w-fit grid grid-cols-7 gap-2", className)}>
      {Object.values(Day).map((day) => (
        <div
          key={day}
          className={`w-full aspect-square rounded-full border-2 border-black flex items-center justify-center cursor-pointer transition-colors ${
            selectedDays.includes(day) ? "bg-primary" : "bg-zinc-200"
          }`}
          onClick={() => toggleDay(day)}
        >
          <div className="w-3/4 h-3/4 rounded-full bg-white flex items-center justify-center border-2 border-black px-5">
            <span className="text-md font-semibold text-black">
              {getDayDisplay(day)}
            </span>
          </div>
        </div>
      ))}
    </div>
  );
};
