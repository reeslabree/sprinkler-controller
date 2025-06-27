import { cn } from "@/lib/utils";

interface Props {
  children: React.ReactNode;
  active?: boolean;
  onClick?: () => void;
}

export const Tab = ({ children, active, onClick }: Props) => {
  return (
    <div
      className={cn(
        "-ml-4 -mr-12 pl-5 rounded-r-full text-secondary-foreground font-bold flex items-center gap-2 text-lg bg-primary py-2 border-t-2 border-b-2 border-r-2 border-black dark:border-white",
        !active &&
          "bg-secondary text-black border-black cursor-pointer dark:text-white"
      )}
      onClick={onClick}
    >
      {children}
    </div>
  );
};
