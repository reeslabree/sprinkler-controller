interface Props {
  children: React.ReactNode;
}

export const SafeAreaPage = ({ children }: Props) => {
  return (
    <div className="w-full h-full min-h-screen flex flex-col items-center pt-10 pb-24">
      <div className="w-full max-w-5xl gap-10 flex flex-1 flex-col h-full">
        <div className="w-full flex h-fit flex-col items-center">
          <div className="w-full flex gap-2 flex-col">
            <h1 className="text-5xl font-extrabold italic">reesync</h1>
            <span className="text-md font-[600]">
              for the discerning lawn owner
            </span>
          </div>
        </div>

        <div className="flex-1 flex flex-grow h-full w-full">{children}</div>
      </div>
    </div>
  );
};
