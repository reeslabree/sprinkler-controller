interface BaseMessage {
  type: string;
  payload: Record<string, unknown>;
}

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

export type ClientMessage = ToggleZonePayload | StatusPayload | StatusResponse;

export type ClientMessageResponse = ToggleZoneResponse | StatusResponse;
