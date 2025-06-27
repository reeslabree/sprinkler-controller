import { Button, Tab } from "@/components";
import { HomeTabs } from "@/constants";
import { SafeAreaPage } from "@/layouts";
import { useState } from "react";
import {
  IoBarChartSharp,
  IoCalendarSharp,
  IoSettingsSharp,
} from "react-icons/io5";

export const HomeScreen = () => {
  const [openTab, setOpenTab] = useState<HomeTabs>(HomeTabs.Data);

  return (
    <SafeAreaPage>
      <div className="w-full border-2 rounded-lg flex min-h-[600px] h-full border-accent-foreground">
        <div className="w-fit flex flex-col px-4 py-8 border-r-2 border-accent-foreground gap-4 min-w-[150px] bg-zinc-300 dark:bg-zinc-800 rounded-l-lg">
          <Tab
            active={openTab === HomeTabs.Data}
            onClick={() => setOpenTab(HomeTabs.Data)}
          >
            <IoBarChartSharp />
            Data
          </Tab>
          <Tab
            active={openTab === HomeTabs.Schedules}
            onClick={() => setOpenTab(HomeTabs.Schedules)}
          >
            <IoCalendarSharp />
            Schedules
          </Tab>
          <Tab
            active={openTab === HomeTabs.Control}
            onClick={() => setOpenTab(HomeTabs.Control)}
          >
            <IoSettingsSharp />
            Control
          </Tab>
        </div>
      </div>
    </SafeAreaPage>
  );
};
