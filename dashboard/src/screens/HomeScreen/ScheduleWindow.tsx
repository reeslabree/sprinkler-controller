import {
  Button,
  DaySelector,
  HourSelector,
  Switch,
  TimePickerInput,
} from "@/components";
import { useWebSocket } from "@/contexts";
import { cn } from "@/lib/utils";
import {
  Day,
  Schedule,
  Schedules,
  SetSchedulePayload,
  Zone,
  ActivePeriod,
} from "@/types";
import debounce from "debounce";
import {
  ChevronLeftIcon,
  ChevronRightIcon,
  EditIcon,
  PlusIcon,
  CheckIcon,
  XIcon,
  Trash2Icon,
} from "lucide-react";
import { useState, useEffect } from "react";

export const ScheduleWindow = () => {
  const {
    sendMessage,
    isClientConnected,
    schedules: serverSchedules,
  } = useWebSocket();

  const [schedules, setSchedules] = useState<Schedules>(serverSchedules || []);

  useEffect(() => {
    if (!isClientConnected) {
      return;
    }

    if (schedules.length > 0) {
      handleSetSchedule(schedules);
    }
  }, [schedules]);

  const [selectedScheduleIndex, setSelectedScheduleIndex] = useState<number>(0);
  const [isEditingName, setIsEditingName] = useState<boolean>(false);
  const [editingName, setEditingName] = useState<string>("");

  const selectedSchedule = schedules[selectedScheduleIndex] || null;
  const isAtEnd = selectedScheduleIndex === schedules.length - 1;
  const isAtStart = selectedScheduleIndex === 0;

  const handleSetSchedule = (schedules: Schedules) => {
    if (isClientConnected) {
      sendMessage<SetSchedulePayload>({
        type: "setSchedule",
        payload: { schedules },
      });
    }
  };

  const handlePreviousSchedule = () => {
    if (!isAtStart) {
      setSelectedScheduleIndex(selectedScheduleIndex - 1);
    }
  };

  const handleNextSchedule = () => {
    if (!isAtEnd) {
      setSelectedScheduleIndex(selectedScheduleIndex + 1);
    }
  };

  const handleAddSchedule = () => {
    const newSchedule: Schedule = {
      name: `Schedule ${schedules.length + 1}`,
      days: [],
      activePeriods: [],
      startTimeMinutes: 0,
      isActive: true,
    };
    const updatedSchedules = [...schedules, newSchedule];
    setSchedules(updatedSchedules);
    setSelectedScheduleIndex(updatedSchedules.length - 1);
    handleSetSchedule(updatedSchedules);
  };

  const handleUpdateSchedule = (updatedSchedule: Schedule) => {
    const updatedSchedules = [...schedules];
    updatedSchedules[selectedScheduleIndex] = updatedSchedule;
    setSchedules(updatedSchedules);
    handleSetSchedule(updatedSchedules);
  };

  const handleStartEditingName = () => {
    setIsEditingName(true);
    setEditingName(selectedSchedule?.name || "");
  };

  const handleSaveName = () => {
    if (!selectedSchedule) return;

    const updatedSchedule = {
      ...selectedSchedule,
      name: editingName.trim() || "Untitled Schedule",
    };

    handleUpdateSchedule(updatedSchedule);
    setIsEditingName(false);
  };

  const handleCancelEditingName = () => {
    setIsEditingName(false);
    setEditingName("");
  };

  const handleUpdateActiveSchedule = (isActive: boolean) => {
    if (!selectedSchedule) return;

    const updatedSchedule = {
      ...selectedSchedule,
      isActive,
    };

    handleUpdateSchedule(updatedSchedule);
  };

  const handleDeleteSchedule = () => {
    if (schedules.length <= 1) return;

    const updatedSchedules = schedules.filter(
      (_, index) => index !== selectedScheduleIndex
    );
    setSchedules(updatedSchedules);

    if (selectedScheduleIndex >= updatedSchedules.length) {
      setSelectedScheduleIndex(updatedSchedules.length - 1);
    }

    handleSetSchedule(updatedSchedules);
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSaveName();
    } else if (e.key === "Escape") {
      handleCancelEditingName();
    }
  };

  const handleUpdateActivePeriod = debounce((activePeriod: ActivePeriod) => {
    if (!selectedSchedule) return;

    const updatedActivePeriods = selectedSchedule.activePeriods.filter(
      (period) => period.zone !== activePeriod.zone
    );

    if (activePeriod.durationMinutes > 0) {
      updatedActivePeriods.push(activePeriod);
    }

    const updatedSchedule = {
      ...selectedSchedule,
      activePeriods: updatedActivePeriods,
    };

    handleUpdateSchedule(updatedSchedule);
  }, 1000);

  const handleUpdateSelectedDays = (days: Day[]) => {
    if (!selectedSchedule) return;

    const updatedSchedule = {
      ...selectedSchedule,
      days,
    };

    handleUpdateSchedule(updatedSchedule);
  };

  const getActivePeriodForZone = (zone: Zone): ActivePeriod | undefined => {
    return selectedSchedule?.activePeriods.find(
      (period) => period.zone === zone
    );
  };

  const getStartTimeDate = (): Date => {
    if (!selectedSchedule) return new Date();
    const minutes = selectedSchedule.startTimeMinutes;
    const date = new Date();
    date.setHours(0, minutes, 0, 0);
    return date;
  };

  const handleUpdateStartTime = (newDate: Date | undefined) => {
    if (!selectedSchedule || !newDate) return;

    const hours = newDate.getHours();
    const minutes = newDate.getMinutes();
    const startTimeMinutes = hours * 60 + minutes;

    const updatedSchedule = {
      ...selectedSchedule,
      startTimeMinutes,
    };

    handleUpdateSchedule(updatedSchedule);
  };

  return (
    <div className="w-full flex-1 flex flex-col justify-center items-center gap-6">
      <div className="w-fit flex justify-between items-center gap-20">
        <button
          onClick={handlePreviousSchedule}
          disabled={isAtStart}
          className={cn(
            "w-6 h-6 transition-opacity",
            isAtStart && "opacity-50 cursor-not-allowed"
          )}
        >
          <ChevronLeftIcon className="w-6 h-6 cursor-pointer" />
        </button>
        <div className="w-fit flex gap-2 items-center">
          {isEditingName ? (
            <div className="flex gap-2 items-center">
              <input
                type="text"
                value={editingName}
                onChange={(e) => setEditingName(e.target.value)}
                onKeyDown={handleKeyPress}
                className="text-lg font-semibold bg-transparent border-b border-gray-300 focus:border-zinc-500 outline-none px-1"
                autoFocus
              />
              <button
                onClick={handleSaveName}
                className="w-4 h-4 transition-opacity hover:opacity-80"
              >
                <CheckIcon className="w-4 h-4 cursor-pointer" />
              </button>
              <button
                onClick={handleCancelEditingName}
                className="w-4 h-4 transition-opacity hover:opacity-80"
              >
                <XIcon className="w-4 h-4 cursor-pointer" />
              </button>
            </div>
          ) : (
            <>
              <span className="text-lg font-semibold">
                {selectedSchedule?.name || "No Schedule"}
              </span>
              <button
                onClick={handleStartEditingName}
                className="w-4 h-4 transition-opacity hover:opacity-80 hover:text-zinc-600 cursor-pointer"
              >
                <EditIcon className="w-4 h-4" />
              </button>
              {schedules.length > 1 && (
                <button
                  onClick={handleDeleteSchedule}
                  className="w-4 h-4 transition-opacity hover:opacity-80 hover:text-destructive cursor-pointer"
                >
                  <Trash2Icon className="w-4 h-4" />
                </button>
              )}
            </>
          )}
        </div>
        {isAtEnd ? (
          <button
            onClick={handleAddSchedule}
            className="w-6 h-6 transition-opacity hover:opacity-80 cursor-pointer"
          >
            <PlusIcon className="w-6 h-6" />
          </button>
        ) : (
          <button
            onClick={handleNextSchedule}
            className="w-6 h-6 transition-opacity cursor-pointer"
          >
            <ChevronRightIcon className="w-6 h-6" />
          </button>
        )}
      </div>
      <div className="flex flex-col justify-center items-center gap-6">
        <div className="flex gap-16 items-center justify-between">
          <div className="flex items-center gap-1">
            <span className="text-md font-semibold">Active</span>
            <Switch
              checked={selectedSchedule?.isActive}
              onCheckedChange={handleUpdateActiveSchedule}
            />
          </div>
          <div className="flex items-center">
            <span className="text-md font-semibold pr-1">Start Time</span>

            <TimePickerInput
              picker="hours"
              date={getStartTimeDate()}
              setDate={handleUpdateStartTime}
            />
            <span className="text-lg font-semibold">:</span>
            <TimePickerInput
              picker="minutes"
              date={getStartTimeDate()}
              setDate={handleUpdateStartTime}
            />
          </div>
        </div>
        {/* <span className="text-lg font-semibold w-full text-center">
          Run Times
        </span> */}
        <div className="w-fit grid grid-cols-3 gap-4">
          <HourSelector
            zone={Zone.Zone1}
            activePeriod={getActivePeriodForZone(Zone.Zone1)}
            setActivePeriod={handleUpdateActivePeriod}
          />
          <HourSelector
            zone={Zone.Zone2}
            activePeriod={getActivePeriodForZone(Zone.Zone2)}
            setActivePeriod={handleUpdateActivePeriod}
          />
          <HourSelector
            zone={Zone.Zone3}
            activePeriod={getActivePeriodForZone(Zone.Zone3)}
            setActivePeriod={handleUpdateActivePeriod}
          />
          <HourSelector
            zone={Zone.Zone4}
            activePeriod={getActivePeriodForZone(Zone.Zone4)}
            setActivePeriod={handleUpdateActivePeriod}
          />
          <HourSelector
            zone={Zone.Zone5}
            activePeriod={getActivePeriodForZone(Zone.Zone5)}
            setActivePeriod={handleUpdateActivePeriod}
          />
          <HourSelector
            zone={Zone.Zone6}
            activePeriod={getActivePeriodForZone(Zone.Zone6)}
            setActivePeriod={handleUpdateActivePeriod}
          />
        </div>
        {/* <span className="text-lg font-semibold w-full text-center">
          Run Days
        </span> */}
        <DaySelector
          selectedDays={selectedSchedule?.days || []}
          setSelectedDays={handleUpdateSelectedDays}
        />
      </div>
    </div>
  );
};

const ScheduleTime = ({
  isActive,
  setIsActive,
}: {
  isActive: boolean;
  setIsActive: (isActive: boolean) => void;
}) => {
  return (
    <div
      className={cn(
        "w-full h-full transition-colors duration-200",
        isActive ? "bg-primary" : "bg-zinc-300"
      )}
    />
  );
};
