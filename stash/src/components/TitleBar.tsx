import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { MouseEvent } from "react";

export function TitleBar() {
  const appWindow = getCurrentWindow();

  async function handleHide() {
    await invoke("hide_panel");
  }

  async function handleTitleBarMouseDown(event: MouseEvent<HTMLElement>) {
    if (event.button !== 0) {
      return;
    }

    if ((event.target as HTMLElement).closest("button")) {
      return;
    }

    event.preventDefault();
    await appWindow.startDragging();
  }

  return (
    <header
      onMouseDown={handleTitleBarMouseDown}
      className="flex cursor-grab select-none items-center justify-between border-b border-black/10 px-4 py-3 active:cursor-grabbing dark:border-white/10"
    >
      <span className="text-[13px] font-semibold tracking-tight text-zinc-900 dark:text-zinc-100">
        Stash
      </span>
      <button
        type="button"
        className="cursor-pointer rounded-md border border-zinc-300/80 bg-white/70 px-2.5 py-1 text-xs text-zinc-900 hover:bg-white/90 dark:border-zinc-600/80 dark:bg-zinc-800/70 dark:text-zinc-100 dark:hover:bg-zinc-800/90"
        onClick={handleHide}
        aria-label="Hide window"
      >
        Hide
      </button>
    </header>
  );
}
