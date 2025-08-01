import {
  ClientMessage,
  ClientMessageResponse,
  GetConfigPayload,
  Schedules,
  ServerMessage,
} from "@/types";
import {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
} from "react";

interface WebSocketContextType {
  ws: WebSocket | null;
  isClientConnected: boolean;
  isControllerConnected: boolean;
  sendMessage: <T extends ClientMessage>(message: T) => void;
  schedules: Schedules;
  staggerOn: boolean;
  staggerZones: boolean;
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(
  undefined
);

interface WebSocketProviderProps {
  children: ReactNode;
}

export const WebSocketProvider = ({ children }: WebSocketProviderProps) => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [isClientConnected, setIsClientConnected] = useState(false);
  const [isControllerConnected, setIsControllerConnected] = useState(false);
  const [schedules, setSchedules] = useState<Schedules>([]);
  const [staggerOn, setStaggerOn] = useState(false);
  const [staggerZones, setStaggerZones] = useState(false);
  const [latestResponse, setLatestResponse] =
    useState<ClientMessageResponse | null>(null);

  const PORT = process.env.NEXT_PUBLIC_WEBSOCKET_PORT;
  const IP = process.env.NEXT_PUBLIC_WEBSOCKET_IP;
  const PATH = process.env.NEXT_PUBLIC_WEBSOCKET_PATH;

  useEffect(() => {
    const websocket = new WebSocket(`ws://${IP}:${PORT}${PATH}`);

    websocket.onopen = () => {
      console.log("connected to server");
      setIsClientConnected(true);

      // identifier string
      websocket.send("user");

      // get config
      const getConfigPayload: GetConfigPayload = {
        type: "getConfig",
        payload: {},
      };
      websocket.send(JSON.stringify(getConfigPayload));
    };

    websocket.onclose = () => {
      console.log("disconnected from server");
      setIsClientConnected(false);
    };

    websocket.onerror = (error) => {
      console.error("WebSocket error:", error);
      setIsClientConnected(false);
    };

    websocket.onmessage = (event) => {
      const data = JSON.parse(event.data) as
        | ServerMessage
        | ClientMessageResponse;
      switch (data.type) {
        case "controllerHeartbeat":
          console.log("Controller heartbeat: ", data.payload);
          const isControllerConnected = data.payload.isControllerConnected;
          setIsControllerConnected(isControllerConnected);
          break;
        case "getConfigResponse":
          console.log("Config: ", data.payload);
          const { schedules, staggerOn, staggerZones } = data.payload;
          setSchedules(schedules);
          setStaggerOn(staggerOn);
          setStaggerZones(staggerZones);
          break;
        case "keepAliveResponse":
        case "toggleZoneResponse":
        case "statusResponse":
        case "setScheduleResponse":
          setLatestResponse(data);
          break;
        default:
          console.log("Unhandled server message: ", data);
      }
    };

    setWs(websocket);

    return () => {
      websocket.close();
    };
  }, [IP, PORT, PATH]);

  const sendMessage = <T extends ClientMessage>(message: T) => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(message));
    } else {
      throw new Error("WebSocket is not connected");
    }
  };

  return (
    <WebSocketContext.Provider
      value={{
        ws,
        isClientConnected,
        isControllerConnected,
        sendMessage,
        schedules,
        staggerOn,
        staggerZones,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => {
  const context = useContext(WebSocketContext);
  if (context === undefined) {
    throw new Error("useWebSocket must be used within a WebSocketProvider");
  }
  return context;
};
