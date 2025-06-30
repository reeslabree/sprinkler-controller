import { useWebSocket } from "@/contexts/WebSocketContext";
import { cn } from "@/lib/utils";
import Head from "next/head";
import { IoEllipse } from "react-icons/io5";

interface Props {
  children: React.ReactNode;
}

const ConnectionStatus = ({
  title,
  isConnected,
}: {
  title: string;
  isConnected: boolean;
}) => {
  return (
    <div
      className={cn(
        "w-fit flex gap-2 items-end items-center",
        isConnected ? "text-primary" : "text-destructive"
      )}
    >
      <span className="text-sm font-[600] whitespace-nowrap">{title}</span>
      <IoEllipse size={10} />
    </div>
  );
};

export const SafeAreaPage = ({ children }: Props) => {
  const { isClientConnected, isControllerConnected } = useWebSocket();

  return (
    <>
      <Head>
        <title>reesync</title>
      </Head>
      <div className="w-full h-full min-h-screen flex flex-col items-center pt-10 pb-24">
        <div className="w-full max-w-5xl gap-10 flex flex-1 flex-col h-full">
          <div className="w-full flex h-fit items-end justify-between">
            <div className="w-full flex gap-2 flex-col">
              <h1 className="text-5xl font-extrabold italic">reesync</h1>
              <span className="text-md font-[600]">
                for the discerning lawn owner
              </span>
            </div>
            <div className="w-fit flex flex-col gap-2 items-end">
              <ConnectionStatus
                title="Client"
                isConnected={isClientConnected}
              />
              <ConnectionStatus
                title="Controller"
                isConnected={isControllerConnected}
              />
            </div>
          </div>

          <div className="flex-1 flex flex-grow h-full w-full">{children}</div>
        </div>
      </div>
    </>
  );
};
