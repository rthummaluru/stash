import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { ActivityFeed } from "./components/ActivityFeed";
import { TitleBar } from "./components/TitleBar";
import type { FileEvent } from "./types/fileEvent";

function App() {
  const [fileEvents, setFileEvents] = useState<FileEvent[]>([]);

  useEffect(() => {
    const unlisten = listen<FileEvent>("file_detected", (event) => {
      setFileEvents((prev) => {
        const withoutDuplicate = prev.filter(
          (item) => item.full_path !== event.payload.full_path,
        );
        return [event.payload, ...withoutDuplicate];
      });
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div className="app">
      <TitleBar />
      <ActivityFeed events={fileEvents} />
    </div>
  );
}

export default App;
