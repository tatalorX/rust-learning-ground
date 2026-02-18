import { useState, useRef, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Play,
  Pause,
  Volume2,
  VolumeX,
  Maximize,
  SkipForward,
  SkipBack,
  Heart,
  MessageCircle,
  Share2,
  MoreHorizontal,
} from "lucide-react";

interface TikTokVideoProps {
  videoId: string;
  videoUrl: string;
  thumbnail?: string;
  duration: string;
  author: string;
  description: string;
  likes: number;
  comments: number;
  shares: number;
  onComplete?: () => void;
}

export function TikTokVideoPlayer({
  videoId,
  videoUrl,
  thumbnail,
  duration,
  author,
  description,
  likes,
  comments,
  shares,
  onComplete,
}: TikTokVideoProps) {
  const [isPlaying, setIsPlaying] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [progress, setProgress] = useState(0);
  const [currentTime, setCurrentTime] = useState("0:00");
  const videoRef = useRef<HTMLVideoElement>(null);

  const formatNumber = (num: number) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num.toString();
  };

  const togglePlay = () => {
    if (videoRef.current) {
      if (isPlaying) {
        videoRef.current.pause();
      } else {
        videoRef.current.play();
      }
      setIsPlaying(!isPlaying);
    }
  };

  const toggleMute = () => {
    if (videoRef.current) {
      videoRef.current.muted = !isMuted;
      setIsMuted(!isMuted);
    }
  };

  const handleTimeUpdate = () => {
    if (videoRef.current) {
      const progress =
        (videoRef.current.currentTime / videoRef.current.duration) * 100;
      setProgress(progress);

      const minutes = Math.floor(videoRef.current.currentTime / 60);
      const seconds = Math.floor(videoRef.current.currentTime % 60);
      setCurrentTime(`${minutes}:${seconds.toString().padStart(2, "0")}`);

      if (videoRef.current.ended) {
        onComplete?.();
      }
    }
  };

  const handleSeek = (e: React.MouseEvent<HTMLDivElement>) => {
    if (videoRef.current) {
      const rect = e.currentTarget.getBoundingClientRect();
      const pos = (e.clientX - rect.left) / rect.width;
      videoRef.current.currentTime = pos * videoRef.current.duration;
    }
  };

  return (
    <motion.div
      initial={{ scale: 0.9, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      className="relative bg-black rounded-3xl overflow-hidden aspect-[9/16] max-w-[400px] mx-auto shadow-2xl"
    >
      {/* Video */}
      <video
        ref={videoRef}
        className="w-full h-full object-cover"
        poster={thumbnail}
        onTimeUpdate={handleTimeUpdate}
        onClick={togglePlay}
        playsInline
      >
        <source src={videoUrl} type="video/mp4" />
      </video>

      {/* Overlay gradient */}
      <div className="absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-black/20 pointer-events-none" />

      {/* Play/Pause overlay */}
      <AnimatePresence>
        {!isPlaying && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 flex items-center justify-center"
            onClick={togglePlay}
          >
            <motion.div
              whileHover={{ scale: 1.1 }}
              whileTap={{ scale: 0.9 }}
              className="w-20 h-20 bg-white/20 backdrop-blur-sm rounded-full flex items-center justify-center"
            >
              <Play className="w-10 h-10 text-white fill-white" />
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Side actions */}
      <div className="absolute right-4 bottom-24 flex flex-col gap-4 items-center">
        <motion.button
          whileTap={{ scale: 0.9 }}
          className="flex flex-col items-center gap-1"
          onClick={() => videoRef.current?.requestPictureInPicture()}
        >
          <div className="w-12 h-12 bg-surface/80 backdrop-blur-sm rounded-full flex items-center justify-center">
            <Heart className="w-6 h-6 text-white" />
          </div>
          <span className="text-xs text-white font-medium">
            {formatNumber(likes)}
          </span>
        </motion.button>

        <motion.button
          whileTap={{ scale: 0.9 }}
          className="flex flex-col items-center gap-1"
        >
          <div className="w-12 h-12 bg-surface/80 backdrop-blur-sm rounded-full flex items-center justify-center">
            <MessageCircle className="w-6 h-6 text-white" />
          </div>
          <span className="text-xs text-white font-medium">
            {formatNumber(comments)}
          </span>
        </motion.button>

        <motion.button
          whileTap={{ scale: 0.9 }}
          className="flex flex-col items-center gap-1"
        >
          <div className="w-12 h-12 bg-surface/80 backdrop-blur-sm rounded-full flex items-center justify-center">
            <Share2 className="w-6 h-6 text-white" />
          </div>
          <span className="text-xs text-white font-medium">
            {formatNumber(shares)}
          </span>
        </motion.button>

        <motion.button whileTap={{ scale: 0.9 }}>
          <div className="w-12 h-12 bg-surface/80 backdrop-blur-sm rounded-full flex items-center justify-center">
            <MoreHorizontal className="w-6 h-6 text-white" />
          </div>
        </motion.button>
      </div>

      {/* Bottom info */}
      <div className="absolute bottom-4 left-4 right-20">
        <div className="mb-2">
          <span className="text-white font-bold text-lg">@{author}</span>
        </div>
        <p className="text-white text-sm line-clamp-2">{description}</p>
      </div>

      {/* Progress bar */}
      <div
        className="absolute bottom-0 left-0 right-0 h-1 bg-white/30 cursor-pointer"
        onClick={handleSeek}
      >
        <motion.div
          className="h-full bg-primary"
          style={{ width: `${progress}%` }}
        />
      </div>

      {/* Time */}
      <div className="absolute bottom-4 right-4">
        <span className="text-white text-xs bg-black/50 px-2 py-1 rounded">
          {currentTime} / {duration}
        </span>
      </div>
    </motion.div>
  );
}

interface VideoGuideCardProps {
  title: string;
  description: string;
  videoThumbnail: string;
  duration: string;
  onClick: () => void;
}

export function VideoGuideCard({
  title,
  description,
  videoThumbnail,
  duration,
  onClick,
}: VideoGuideCardProps) {
  return (
    <motion.div
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
      onClick={onClick}
      className="bg-surface border border-border-subtle rounded-xl overflow-hidden cursor-pointer"
    >
      <div className="relative aspect-video">
        <img
          src={videoThumbnail}
          alt={title}
          className="w-full h-full object-cover"
        />
        <div className="absolute inset-0 bg-black/30 flex items-center justify-center">
          <motion.div
            whileHover={{ scale: 1.1 }}
            className="w-12 h-12 bg-white/20 backdrop-blur-sm rounded-full flex items-center justify-center"
          >
            <Play className="w-6 h-6 text-white fill-white" />
          </motion.div>
        </div>
        <div className="absolute bottom-2 right-2 bg-black/70 text-white text-xs px-2 py-1 rounded">
          {duration}
        </div>
      </div>
      <div className="p-4">
        <h4 className="font-bold text-text-primary mb-1">{title}</h4>
        <p className="text-sm text-text-secondary line-clamp-2">
          {description}
        </p>
      </div>
    </motion.div>
  );
}

interface VideoGridProps {
  videos: Array<{
    id: string;
    title: string;
    description: string;
    thumbnail: string;
    duration: string;
    onClick: () => void;
  }>;
}

export function VideoGrid({ videos }: VideoGridProps) {
  return (
    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      {videos.map((video) => (
        <VideoGuideCard
          key={video.id}
          title={video.title}
          description={video.description}
          videoThumbnail={video.thumbnail}
          duration={video.duration}
          onClick={video.onClick}
        />
      ))}
    </div>
  );
}
