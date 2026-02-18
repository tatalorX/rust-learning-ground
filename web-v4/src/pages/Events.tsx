import { useState } from "react";
import { motion } from "framer-motion";
import { useEventsStore } from "../stores/events";
import { useEngagementStore } from "../stores/engagement";
import {
  Calendar,
  Trophy,
  Users,
  Clock,
  CheckCircle,
  AlertCircle,
  Zap,
  Award,
  Flame,
  Target,
} from "lucide-react";

const eventTypeIcons = {
  hackathon: Trophy,
  challenge: Target,
  workshop: Award,
  competition: Flame,
};

const eventTypeColors = {
  hackathon: "from-purple-500 to-pink-500",
  challenge: "from-green-500 to-emerald-500",
  workshop: "from-blue-500 to-cyan-500",
  competition: "from-orange-500 to-red-500",
};

export default function EventsPage() {
  const [activeTab, setActiveTab] = useState<"active" | "upcoming" | "past">(
    "active",
  );
  const { events, registerForEvent, unregisterFromEvent, completedEventIds } =
    useEventsStore();
  const engagementStore = useEngagementStore();

  const now = new Date();

  const filteredEvents = events.filter((event) => {
    const start = new Date(event.startDate);
    const end = new Date(event.endDate);

    if (activeTab === "active") {
      return start <= now && end >= now;
    } else if (activeTab === "upcoming") {
      return start > now;
    } else {
      return end < now;
    }
  });

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString("en-US", {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  const getTimeRemaining = (endDate: string) => {
    const end = new Date(endDate);
    const diff = end.getTime() - now.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));

    if (days > 0) return `${days}d ${hours}h remaining`;
    if (hours > 0) return `${hours}h remaining`;
    return "Ending soon";
  };

  return (
    <div className="space-y-8">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-display font-bold text-text-primary flex items-center gap-3">
            <Calendar className="w-8 h-8 text-primary" />
            Events & Challenges
          </h1>
          <p className="text-text-secondary mt-1">
            Join hackathons, challenges, and workshops to earn bonus XP
          </p>
        </div>

        <div className="flex items-center gap-2 p-1 rounded-xl bg-surface border border-border-subtle">
          {(["active", "upcoming", "past"] as const).map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-all capitalize ${
                activeTab === tab
                  ? "bg-primary text-white"
                  : "text-text-secondary hover:text-text-primary"
              }`}
            >
              {tab}
            </button>
          ))}
        </div>
      </div>

      {/* Stats */}
      <div className="grid sm:grid-cols-3 gap-4">
        <div className="p-6 rounded-2xl bg-surface border border-border-subtle">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center">
              <Trophy className="w-5 h-5 text-primary" />
            </div>
            <span className="text-text-secondary">Events Completed</span>
          </div>
          <p className="text-3xl font-display font-bold text-text-primary">
            {completedEventIds.length}
          </p>
        </div>

        <div className="p-6 rounded-2xl bg-surface border border-border-subtle">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-10 h-10 rounded-xl bg-success/10 flex items-center justify-center">
              <Zap className="w-5 h-5 text-success" />
            </div>
            <span className="text-text-secondary">XP from Events</span>
          </div>
          <p className="text-3xl font-display font-bold text-text-primary">
            {completedEventIds.length * 250}
          </p>
        </div>

        <div className="p-6 rounded-2xl bg-surface border border-border-subtle">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-10 h-10 rounded-xl bg-orange-500/10 flex items-center justify-center">
              <Users className="w-5 h-5 text-orange-500" />
            </div>
            <span className="text-text-secondary">Registered</span>
          </div>
          <p className="text-3xl font-display font-bold text-text-primary">
            {events.filter((e) => e.isRegistered).length}
          </p>
        </div>
      </div>

      {/* Events List */}
      <div className="grid gap-6">
        {filteredEvents.length === 0 ? (
          <div className="text-center py-12">
            <Calendar className="w-16 h-16 mx-auto text-text-muted mb-4" />
            <h3 className="text-lg font-semibold text-text-primary mb-2">
              No {activeTab} events
            </h3>
            <p className="text-text-secondary">
              Check back later for new events and challenges
            </p>
          </div>
        ) : (
          filteredEvents.map((event, index) => {
            const Icon = eventTypeIcons[event.type];
            const colorClass = eventTypeColors[event.type];
            const isFull =
              event.registeredParticipants >= event.maxParticipants;
            const isCompleted = completedEventIds.includes(event.id);

            return (
              <motion.div
                key={event.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.1 }}
                className={`p-6 rounded-2xl border transition-all ${
                  isCompleted
                    ? "bg-success/5 border-success/30"
                    : event.isRegistered
                      ? "bg-primary/5 border-primary/30"
                      : "bg-surface border-border-subtle hover:border-primary/30"
                }`}
              >
                <div className="flex flex-col lg:flex-row lg:items-start gap-6">
                  {/* Icon */}
                  <div
                    className={`w-16 h-16 rounded-2xl bg-gradient-to-br ${colorClass} flex items-center justify-center flex-shrink-0`}
                  >
                    {isCompleted ? (
                      <CheckCircle className="w-8 h-8 text-white" />
                    ) : (
                      <Icon className="w-8 h-8 text-white" />
                    )}
                  </div>

                  {/* Content */}
                  <div className="flex-1 min-w-0">
                    <div className="flex flex-wrap items-center gap-3 mb-2">
                      <h3 className="text-xl font-bold text-text-primary">
                        {event.title}
                      </h3>
                      <span
                        className={`px-2 py-1 rounded-full text-xs font-medium capitalize ${
                          event.difficulty === "beginner"
                            ? "bg-green-500/20 text-green-400"
                            : event.difficulty === "intermediate"
                              ? "bg-yellow-500/20 text-yellow-400"
                              : "bg-red-500/20 text-red-400"
                        }`}
                      >
                        {event.difficulty}
                      </span>
                      {isCompleted && (
                        <span className="px-2 py-1 rounded-full text-xs font-medium bg-success/20 text-success">
                          Completed
                        </span>
                      )}
                      {event.isRegistered && !isCompleted && (
                        <span className="px-2 py-1 rounded-full text-xs font-medium bg-primary/20 text-primary">
                          Registered
                        </span>
                      )}
                    </div>

                    <p className="text-text-secondary mb-4">
                      {event.description}
                    </p>

                    {/* Requirements */}
                    <div className="flex flex-wrap gap-2 mb-4">
                      {event.requirements.map((req, i) => (
                        <span
                          key={i}
                          className="px-3 py-1 rounded-full text-xs bg-surface-hover text-text-secondary"
                        >
                          {req}
                        </span>
                      ))}
                    </div>

                    {/* Meta */}
                    <div className="flex flex-wrap items-center gap-4 text-sm text-text-muted mb-4">
                      <span className="flex items-center gap-1">
                        <Clock className="w-4 h-4" />
                        {formatDate(event.startDate)}
                      </span>
                      <span className="flex items-center gap-1">
                        <Users className="w-4 h-4" />
                        {event.registeredParticipants}/{event.maxParticipants}{" "}
                        registered
                      </span>
                      <span className="flex items-center gap-1 text-primary font-medium">
                        <Zap className="w-4 h-4" />+{event.xpReward} XP
                      </span>
                    </div>

                    {/* Prizes */}
                    {event.prizes && event.prizes.length > 0 && (
                      <div className="p-3 rounded-xl bg-gradient-to-r from-yellow-500/10 to-orange-500/10 border border-yellow-500/20 mb-4">
                        <p className="text-sm font-medium text-text-primary mb-1">
                          Prizes:
                        </p>
                        <ul className="text-sm text-text-secondary space-y-1">
                          {event.prizes.map((prize, i) => (
                            <li key={i}>{prize}</li>
                          ))}
                        </ul>
                      </div>
                    )}

                    {/* Time remaining */}
                    {activeTab === "active" && (
                      <div className="flex items-center gap-2 text-sm text-orange-400">
                        <AlertCircle className="w-4 h-4" />
                        {getTimeRemaining(event.endDate)}
                      </div>
                    )}
                  </div>

                  {/* Actions */}
                  <div className="flex flex-col gap-2">
                    {!isCompleted && (
                      <>
                        {event.isRegistered ? (
                          <button
                            onClick={() => unregisterFromEvent(event.id)}
                            className="px-6 py-3 rounded-xl border border-border-subtle text-text-secondary font-medium hover:text-text-primary hover:border-text-primary transition-all"
                          >
                            Unregister
                          </button>
                        ) : (
                          <button
                            onClick={() => registerForEvent(event.id)}
                            disabled={isFull}
                            className="px-6 py-3 rounded-xl bg-primary text-white font-medium hover:bg-primary-hover transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                          >
                            {isFull ? "Full" : "Register"}
                          </button>
                        )}
                      </>
                    )}
                    {isCompleted && (
                      <div className="px-6 py-3 rounded-xl bg-success/20 text-success font-medium flex items-center gap-2">
                        <CheckCircle className="w-5 h-5" />
                        Completed
                      </div>
                    )}
                  </div>
                </div>
              </motion.div>
            );
          })
        )}
      </div>

      {/* Call to action */}
      {engagementStore.getLevel() < 5 && (
        <div className="p-6 rounded-2xl bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20 text-center">
          <h3 className="text-xl font-bold text-text-primary mb-2">
            Unlock More Events
          </h3>
          <p className="text-text-secondary mb-4">
            Reach Level 5 to unlock advanced hackathons and competitions
          </p>
          <div className="w-full max-w-xs mx-auto h-3 bg-surface rounded-full overflow-hidden">
            <div
              className="h-full bg-primary transition-all"
              style={{
                width: `${Math.min(100, (engagementStore.xp / 250) * 100)}%`,
              }}
            />
          </div>
          <p className="text-sm text-text-muted mt-2">
            {engagementStore.xp} / 250 XP to Level 5
          </p>
        </div>
      )}
    </div>
  );
}
