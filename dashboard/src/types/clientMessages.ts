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

export type ClientMessage =
  | KeepAlivePayload
  | ToggleZonePayload
  | StatusPayload;

export type ClientMessageResponse =
  | KeepAliveResponse
  | ToggleZoneResponse
  | StatusResponse;
