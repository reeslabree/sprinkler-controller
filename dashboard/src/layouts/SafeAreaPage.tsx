interface Props {
  children: React.ReactNode;
}

export const SafeAreaPage = ({ children }: Props) => {
  return (
    <div className="w-full h-full min-h-screen flex flex-col items-center py-10">
      <div className="w-full max-w-5xl gap-10 flex flex-col h-full">
        <div className="w-full flex h-fit flex-col items-center">
          <div className="w-full flex gap-2 flex-col">
            <h1 className="text-5xl font-extrabold italic">reesync</h1>
            <span className="text-md font-[600]">
              for the discerning lawn owner
            </span>
          </div>
        </div>

        {children}
      </div>
    </div>
  );
};
