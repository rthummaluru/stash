import type { FileEvent } from "../types/fileEvent";

function formatRelativeTime(iso: string): string {
  const diffMs = Date.now() - new Date(iso).getTime();
  const diffSec = Math.floor(diffMs / 1000);

  if (diffSec < 10) return "just now";
  if (diffSec < 60) return `${diffSec}s ago`;

  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60) return `${diffMin}m ago`;

  const diffHr = Math.floor(diffMin / 60);
  if (diffHr < 24) return `${diffHr}h ago`;

  return new Date(iso).toLocaleDateString();
}

interface ActivityItemProps {
  event: FileEvent;
}

export function ActivityItem({ event }: ActivityItemProps) {
  return (
    <li className="activity-item">
      <div className="activity-item__dot" aria-hidden="true" />
      <div className="activity-item__content">
        <span className="activity-item__name">{event.file_name}</span>
        <span className="activity-item__meta">
          {event.source_folder} · {formatRelativeTime(event.detected_at)}
        </span>
      </div>
    </li>
  );
}
