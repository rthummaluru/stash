import { TitleBar } from "./TitleBar";

export function PanelShell() {
  return (
    <div className="flex h-full max-h-[80vh] w-full flex-col overflow-hidden rounded-xl border border-black/10 shadow-2xl dark:border-white/10">
      <TitleBar />
      <main className="flex flex-1 flex-col items-center justify-center px-6 py-10 text-center">
        <p className="text-sm font-medium text-zinc-800 dark:text-zinc-200">
          Stash is running
        </p>
        <p className="mt-2 max-w-[240px] text-xs leading-relaxed text-zinc-500 dark:text-zinc-400">
          Panel coming soon. Use the tray menu or hotkey to summon this window.
        </p>
      </main>
    </div>
  );
}
