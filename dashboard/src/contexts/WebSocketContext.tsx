import {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
} from "react";

interface WebSocketContextType {
  ws: WebSocket | null;
  isConnected: boolean;
  sendMessage: (message: string) => void;
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(
  undefined
);

interface WebSocketProviderProps {
  children: ReactNode;
}

export const WebSocketProvider = ({ children }: WebSocketProviderProps) => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  const PORT = process.env.NEXT_PUBLIC_WEBSOCKET_PORT;
  const IP = process.env.NEXT_PUBLIC_WEBSOCKET_IP;
  const PATH = process.env.NEXT_PUBLIC_WEBSOCKET_PATH;

  useEffect(() => {
    const websocket = new WebSocket(`ws://${IP}:${PORT}${PATH}`);

    websocket.onopen = () => {
      console.log("connected to server");
      setIsConnected(true);
      websocket.send("user");
    };

    websocket.onclose = () => {
      console.log("disconnected from server");
      setIsConnected(false);
    };

    websocket.onerror = (error) => {
      console.error("WebSocket error:", error);
      setIsConnected(false);
    };

    setWs(websocket);

    return () => {
      websocket.close();
    };
  }, [IP, PORT, PATH]);

  const sendMessage = (message: string) => {
    if (ws && isConnected) {
      ws.send(message);
    } else {
      console.warn("WebSocket is not connected");
    }
  };

  return (
    <WebSocketContext.Provider value={{ ws, isConnected, sendMessage }}>
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
