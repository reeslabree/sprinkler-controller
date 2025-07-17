export enum Day {
  Monday = "monday",
  Tuesday = "tuesday",
  Wednesday = "wednesday",
  Thursday = "thursday",
  Friday = "friday",
  Saturday = "saturday",
  Sunday = "sunday",
}

export enum Zone {
  Zone1 = "zone1",
  Zone2 = "zone2",
  Zone3 = "zone3",
  Zone4 = "zone4",
  Zone5 = "zone5",
  Zone6 = "zone6",
}

export interface ActivePeriod {
  zone: Zone;
  durationMinutes: number;
}

export interface Schedule {
  name: string;
  days: Day[];
  activePeriods: ActivePeriod[];
  startTimeMinutes: number;
  isActive: boolean;
}

export type Schedules = Schedule[];
