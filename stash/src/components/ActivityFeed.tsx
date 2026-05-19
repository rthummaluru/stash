import type { FileEvent } from "../types/fileEvent";
import { ActivityItem } from "./ActivityItem";

interface ActivityFeedProps {
  events: FileEvent[];
}

export function ActivityFeed({ events }: ActivityFeedProps) {
  return (
    <section className="activity-feed">
      <p className="activity-feed__status">Watching ~/Downloads</p>
      <hr className="activity-feed__divider" />
      {events.length === 0 ? (
        <p className="activity-feed__empty">
          Download a file to test — it&apos;ll show up here.
        </p>
      ) : (
        <ul className="activity-feed__list">
          {events.map((event) => (
            <ActivityItem key={event.id} event={event} />
          ))}
        </ul>
      )}
    </section>
  );
}
