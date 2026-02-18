import { create } from "zustand";
import { persist } from "zustand/middleware";

export type EventType = "hackathon" | "challenge" | "workshop" | "competition";

export interface Event {
  id: string;
  title: string;
  description: string;
  type: EventType;
  startDate: string;
  endDate: string;
  maxParticipants: number;
  registeredParticipants: number;
  xpReward: number;
  requirements: string[];
  prizes?: string[];
  isRegistered: boolean;
  difficulty: "beginner" | "intermediate" | "advanced";
}

interface EventsState {
  events: Event[];
  registeredEventIds: string[];
  completedEventIds: string[];

  // Actions
  registerForEvent: (eventId: string) => void;
  unregisterFromEvent: (eventId: string) => void;
  completeEvent: (eventId: string) => void;
  getActiveEvents: () => Event[];
  getUpcomingEvents: () => Event[];
  getPastEvents: () => Event[];
}

const DEFAULT_EVENTS: Event[] = [
  {
    id: "weekly-challenge-1",
    title: "Weekly Coding Challenge",
    description:
      "Solve 5 medium-difficulty exercises this week to earn bonus XP and a special badge.",
    type: "challenge",
    startDate: new Date().toISOString(),
    endDate: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
    maxParticipants: 1000,
    registeredParticipants: 245,
    xpReward: 100,
    requirements: [
      "Complete 5 medium exercises",
      "Maintain streak during event",
    ],
    difficulty: "intermediate",
    isRegistered: false,
  },
  {
    id: "ownership-mastery",
    title: "Ownership Mastery Workshop",
    description:
      "Deep dive into Rust's ownership system. Live coding session with expert guidance.",
    type: "workshop",
    startDate: new Date(Date.now() + 2 * 24 * 60 * 60 * 1000).toISOString(),
    endDate: new Date(
      Date.now() + 2 * 24 * 60 * 60 * 1000 + 2 * 60 * 60 * 1000,
    ).toISOString(),
    maxParticipants: 50,
    registeredParticipants: 32,
    xpReward: 200,
    requirements: [
      "Completed Ownership chapter in Rust Book",
      "Basic Rust knowledge",
    ],
    difficulty: "intermediate",
    isRegistered: false,
  },
  {
    id: "beginner-hackathon",
    title: "Beginner's First Hackathon",
    description:
      "48-hour hackathon for beginners. Build your first Rust project with mentors available.",
    type: "hackathon",
    startDate: new Date(Date.now() + 5 * 24 * 60 * 60 * 1000).toISOString(),
    endDate: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
    maxParticipants: 200,
    registeredParticipants: 89,
    xpReward: 500,
    requirements: ["Level 5 or below", "First hackathon participation"],
    prizes: [
      "1st: $100 Amazon Gift Card",
      "2nd: $50 Gift Card",
      "3rd: Rust Merch Package",
    ],
    difficulty: "beginner",
    isRegistered: false,
  },
  {
    id: "algo-competition",
    title: "Algorithm Optimization Competition",
    description:
      "Optimize provided algorithms for speed. Fastest solutions win!",
    type: "competition",
    startDate: new Date(Date.now() + 3 * 24 * 60 * 60 * 1000).toISOString(),
    endDate: new Date(Date.now() + 4 * 24 * 60 * 60 * 1000).toISOString(),
    maxParticipants: 500,
    registeredParticipants: 156,
    xpReward: 300,
    requirements: ["Level 10+", "Completed 50+ exercises"],
    prizes: ["1st: $200 + Exclusive Badge", "2nd: $100", "3rd: $50"],
    difficulty: "advanced",
    isRegistered: false,
  },
  {
    id: "rust-30-days",
    title: "30 Days of Rust",
    description:
      "Commit to learning Rust for 30 days straight. Daily exercises and accountability.",
    type: "challenge",
    startDate: new Date(Date.now() + 1 * 24 * 60 * 60 * 1000).toISOString(),
    endDate: new Date(Date.now() + 31 * 24 * 60 * 60 * 1000).toISOString(),
    maxParticipants: 500,
    registeredParticipants: 412,
    xpReward: 1000,
    requirements: [
      "Complete at least 1 exercise per day",
      "Maintain 30-day streak",
    ],
    prizes: [
      "Certificate of Completion",
      "Exclusive '30 Days' Badge",
      "Entry into raffle for Rust book",
    ],
    difficulty: "beginner",
    isRegistered: false,
  },
  {
    id: "concurrency-deep-dive",
    title: "Concurrency Deep Dive",
    description:
      "Advanced workshop on Rust's concurrency features: threads, channels, async/await.",
    type: "workshop",
    startDate: new Date(Date.now() + 10 * 24 * 60 * 60 * 1000).toISOString(),
    endDate: new Date(
      Date.now() + 10 * 24 * 60 * 60 * 1000 + 3 * 60 * 60 * 1000,
    ).toISOString(),
    maxParticipants: 30,
    registeredParticipants: 12,
    xpReward: 400,
    requirements: ["Level 20+", "Completed Concurrency chapter"],
    difficulty: "advanced",
    isRegistered: false,
  },
];

export const useEventsStore = create<EventsState>()(
  persist(
    (set, get) => ({
      events: DEFAULT_EVENTS,
      registeredEventIds: [],
      completedEventIds: [],

      registerForEvent: (eventId) => {
        set((state) => ({
          registeredEventIds: [...state.registeredEventIds, eventId],
          events: state.events.map((e) =>
            e.id === eventId
              ? {
                  ...e,
                  isRegistered: true,
                  registeredParticipants: e.registeredParticipants + 1,
                }
              : e,
          ),
        }));
      },

      unregisterFromEvent: (eventId) => {
        set((state) => ({
          registeredEventIds: state.registeredEventIds.filter(
            (id) => id !== eventId,
          ),
          events: state.events.map((e) =>
            e.id === eventId
              ? {
                  ...e,
                  isRegistered: false,
                  registeredParticipants: Math.max(
                    0,
                    e.registeredParticipants - 1,
                  ),
                }
              : e,
          ),
        }));
      },

      completeEvent: (eventId) => {
        set((state) => ({
          completedEventIds: [...state.completedEventIds, eventId],
        }));
      },

      getActiveEvents: () => {
        const now = new Date().toISOString();
        return get().events.filter(
          (e) => e.startDate <= now && e.endDate >= now,
        );
      },

      getUpcomingEvents: () => {
        const now = new Date().toISOString();
        return get().events.filter((e) => e.startDate > now);
      },

      getPastEvents: () => {
        const now = new Date().toISOString();
        return get().events.filter((e) => e.endDate < now);
      },
    }),
    {
      name: "events-storage",
    },
  ),
);
