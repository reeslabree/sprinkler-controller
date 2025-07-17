import { Schedules } from "./schedules";

interface BaseMessage {
  type: string;
  payload: Record<string, unknown>;
}

// Keep Alive
export interface KeepAlivePayload extends BaseMessage {
  type: "keepAlive";
  payload: {};
}

export interface KeepAliveResponse extends BaseMessage {
  type: "keepAliveResponse";
  payload: {};
}

// Toggle Zone
export interface ToggleZonePayload extends BaseMessage {
  type: "toggleZone";
  payload: {
    zone: number;
    activate: boolean;
  };
}

export interface ToggleZoneResponse extends BaseMessage {
  type: "toggleZoneResponse";
  payload: {
    success: boolean;
    error?: string;
  };
}

// Status
export interface StatusPayload extends BaseMessage {
  type: "status";
  payload: {};
}

export interface StatusResponse extends BaseMessage {
  type: "statusResponse";
  payload: {
    isControllerConnected: boolean;
  };
}

// Set Schedule
export interface SetSchedulePayload extends BaseMessage {
  type: "setSchedule";
  payload: {
    schedules: Schedules;
  };
}

export interface SetScheduleResponse extends BaseMessage {
  type: "setScheduleResponse";
  payload: {
    success: boolean;
    error?: string;
  };
}

// Get Config
export interface GetConfigPayload extends BaseMessage {
  type: "getConfig";
  payload: {};
}

export interface GetConfigResponse extends BaseMessage {
  type: "getConfigResponse";
  payload: {
    schedules: Schedules;
    staggerOn: boolean;
    staggerZones: boolean;
  };
}

// Generics
export type ClientMessage =
  | KeepAlivePayload
  | ToggleZonePayload
  | StatusPayload
  | SetSchedulePayload
  | GetConfigPayload;

export type ClientMessageResponse =
  | KeepAliveResponse
  | ToggleZoneResponse
  | StatusResponse
  | SetScheduleResponse
  | GetConfigResponse;
