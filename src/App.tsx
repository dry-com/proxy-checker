import "./App.css";

import { ThemeProvider } from "@/components/theme-provider";
import { Controls } from "@/components/controls";

export const App = () => {
  return (
    <ThemeProvider>
      <div className="flex">
        <Controls />
        <div className="grid justify-center w-full h-full items-center min-h-[600px]"></div>
      </div>
    </ThemeProvider>
  );
};
