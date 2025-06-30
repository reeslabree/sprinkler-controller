import {
  ClientMessage,
  ClientMessageResponse,
  StatusPayload,
  StatusResponse,
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
  sendMessage: <T extends ClientMessage, R extends ClientMessageResponse>(
    message: T
  ) => Promise<R>;
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(
  undefined
);

interface WebSocketProviderProps {
  children: ReactNode;
}

const sendMessageGeneric = async <
  T extends ClientMessage,
  R extends ClientMessageResponse
>(
  ws: WebSocket,
  message: T
): Promise<R> => {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(message));

    return new Promise((resolve, reject) => {
      ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        resolve(data);
      };
    });
  } else {
    throw new Error("WebSocket is not connected");
  }
};

export const WebSocketProvider = ({ children }: WebSocketProviderProps) => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [isClientConnected, setIsClientConnected] = useState(false);
  const [isControllerConnected, setIsControllerConnected] = useState(false);

  const PORT = process.env.NEXT_PUBLIC_WEBSOCKET_PORT;
  const IP = process.env.NEXT_PUBLIC_WEBSOCKET_IP;
  const PATH = process.env.NEXT_PUBLIC_WEBSOCKET_PATH;

  useEffect(() => {
    const websocket = new WebSocket(`ws://${IP}:${PORT}${PATH}`);

    websocket.onopen = () => {
      console.log("connected to server");
      setIsClientConnected(true);
      websocket.send("user");
      sendMessageGeneric<StatusPayload, StatusResponse>(websocket, {
        type: "status",
        payload: {},
      }).then((data) => {
        console.log(data);
        setIsControllerConnected(data.payload.isControllerConnected);
      });
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
      const data = JSON.parse(event.data);
    };

    setWs(websocket);

    return () => {
      websocket.close();
    };
  }, [IP, PORT, PATH]);

  const sendMessage = <
    T extends ClientMessage,
    R extends ClientMessageResponse
  >(
    message: T
  ): Promise<R> => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(message));

      return new Promise((resolve, reject) => {
        ws.onmessage = (event) => {
          const data = JSON.parse(event.data);
          console.log(data);
          resolve(data);
        };
      });
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
