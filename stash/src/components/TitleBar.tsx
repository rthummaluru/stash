import { getCurrentWindow } from "@tauri-apps/api/window";

export function TitleBar() {
  async function handleHide() {
    await getCurrentWindow().hide();
  }

  return (
    <header className="title-bar">
      <span className="title-bar__name">Stash</span>
      <button
        type="button"
        className="title-bar__hide"
        onClick={handleHide}
        aria-label="Hide window"
      >
        Hide
      </button>
    </header>
  );
}
