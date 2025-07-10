export interface BaseMessage {
  type: string;
  payload: Record<string, unknown>;
}

export interface ControllerHeartbeatPayload extends BaseMessage {
  type: "controllerHeartbeat";
  payload: {
    isControllerConnected: boolean;
  };
}

export type ServerMessage = ControllerHeartbeatPayload;
