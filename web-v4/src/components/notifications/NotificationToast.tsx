import { useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  useNotificationsStore,
  type Notification,
} from "../../stores/notifications";
import { Trophy, Flame, TrendingUp, Bell, X } from "lucide-react";

export function NotificationToast() {
  const { notifications, removeNotification } = useNotificationsStore();

  // Filter for recent non-read notifications (last 5 seconds)
  const recentNotifications = notifications.filter(
    (n: Notification) =>
      !n.read && new Date().getTime() - new Date(n.createdAt).getTime() < 5000,
  );

  const getIcon = (type: string) => {
    switch (type) {
      case "achievement":
        return <Trophy className="w-5 h-5 text-yellow-500" />;
      case "streak":
        return <Flame className="w-5 h-5 text-orange-500" />;
      case "level_up":
        return <TrendingUp className="w-5 h-5 text-primary" />;
      default:
        return <Bell className="w-5 h-5 text-text-secondary" />;
    }
  };

  return (
    <div className="fixed top-20 right-4 z-50 space-y-2 pointer-events-none">
      <AnimatePresence>
        {recentNotifications.map((notification: Notification) => (
          <motion.div
            key={notification.id}
            initial={{ opacity: 0, x: 100, scale: 0.9 }}
            animate={{ opacity: 1, x: 0, scale: 1 }}
            exit={{ opacity: 0, x: 100, scale: 0.9 }}
            className="pointer-events-auto flex items-start gap-3 p-4 rounded-xl bg-surface border border-border-subtle shadow-lg max-w-sm"
          >
            <div className="flex-shrink-0 w-10 h-10 rounded-full bg-surface-hover flex items-center justify-center">
              {getIcon(notification.type)}
            </div>
            <div className="flex-1 min-w-0">
              <p className="font-medium text-text-primary">
                {notification.title}
              </p>
              <p className="text-sm text-text-secondary">
                {notification.message}
              </p>
            </div>
            <button
              onClick={() => removeNotification(notification.id)}
              className="flex-shrink-0 p-1 rounded-lg text-text-muted hover:text-text-primary hover:bg-surface-hover transition-colors"
            >
              <X className="w-4 h-4" />
            </button>
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}

export function NotificationBell() {
  const { unreadCount, notifications, markAsRead, markAllAsRead } =
    useNotificationsStore();
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="relative p-2 rounded-lg text-text-secondary hover:text-text-primary hover:bg-surface-hover transition-all"
      >
        <Bell className="w-5 h-5" />
        {unreadCount > 0 && (
          <span className="absolute -top-1 -right-1 w-5 h-5 rounded-full bg-primary text-white text-xs font-bold flex items-center justify-center">
            {unreadCount > 9 ? "9+" : unreadCount}
          </span>
        )}
      </button>

      {isOpen && (
        <>
          <div
            className="fixed inset-0 z-40"
            onClick={() => setIsOpen(false)}
          />
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            className="absolute right-0 top-full mt-2 w-80 max-h-96 bg-surface border border-border-subtle rounded-xl shadow-lg z-50 overflow-hidden"
          >
            <div className="flex items-center justify-between p-4 border-b border-border-subtle">
              <h3 className="font-semibold text-text-primary">Notifications</h3>
              {unreadCount > 0 && (
                <button
                  onClick={markAllAsRead}
                  className="text-xs text-primary hover:text-primary-hover"
                >
                  Mark all read
                </button>
              )}
            </div>

            <div className="overflow-y-auto max-h-72">
              {notifications.length === 0 ? (
                <div className="p-8 text-center text-text-secondary">
                  <Bell className="w-12 h-12 mx-auto mb-3 text-text-muted" />
                  <p>No notifications yet</p>
                </div>
              ) : (
                notifications.map((notification: Notification) => (
                  <div
                    key={notification.id}
                    onClick={() => markAsRead(notification.id)}
                    className={`p-4 border-b border-border-subtle hover:bg-surface-hover cursor-pointer transition-colors ${
                      !notification.read ? "bg-primary/5" : ""
                    }`}
                  >
                    <div className="flex items-start gap-3">
                      <div className="flex-shrink-0">
                        {notification.type === "achievement" && (
                          <Trophy className="w-5 h-5 text-yellow-500" />
                        )}
                        {notification.type === "streak" && (
                          <Flame className="w-5 h-5 text-orange-500" />
                        )}
                        {notification.type === "level_up" && (
                          <TrendingUp className="w-5 h-5 text-primary" />
                        )}
                      </div>
                      <div className="flex-1 min-w-0">
                        <p className="font-medium text-text-primary text-sm">
                          {notification.title}
                        </p>
                        <p className="text-xs text-text-secondary mt-0.5">
                          {notification.message}
                        </p>
                        <p className="text-xs text-text-muted mt-1">
                          {new Date(
                            notification.createdAt,
                          ).toLocaleDateString()}
                        </p>
                      </div>
                      {!notification.read && (
                        <div className="w-2 h-2 rounded-full bg-primary flex-shrink-0" />
                      )}
                    </div>
                  </div>
                ))
              )}
            </div>
          </motion.div>
        </>
      )}
    </div>
  );
}

import { useState } from "react";
