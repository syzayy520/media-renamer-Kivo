// features/scan/components/DirectoryInput.tsx — 目录路径输入
// 职责：手动输入目录路径，空路径时 Start 按钮 disabled

interface DirectoryInputProps {
  value: string;
  onChange: (value: string) => void;
}

export function DirectoryInput({ value, onChange }: DirectoryInputProps) {
  return (
    <div className="flex gap-3">
      <input
        type="text"
        aria-label="媒体文件目录路径"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="D:\Media\Anime 或 /Users/me/Movies"
        className="min-w-0 flex-1 rounded-md border border-white/10 bg-white/5 px-4 py-2 text-sm text-white placeholder-white/30 outline-none transition-colors focus:border-blue-500/50 focus:bg-white/8"
      />
      <button
        type="button"
        disabled
        className="shrink-0 rounded-md border border-white/10 bg-white/5 px-4 py-2 text-sm text-white/30"
      >
        浏览（后续接入）
      </button>
    </div>
  );
}
