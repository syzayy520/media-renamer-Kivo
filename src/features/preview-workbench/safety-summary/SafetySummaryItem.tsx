// features/preview-workbench/safety-summary/SafetySummaryItem.tsx — 安全摘要单项
// 职责：渲染单个安全摘要信息项

interface SafetySummaryItemProps {
  /** 标签 */
  label: string;
  /** 值 */
  value: string | number | boolean;
  /** 状态颜色 */
  statusColor?: string;
  /** 是否为布尔值 */
  isBoolean?: boolean;
}

export function SafetySummaryItem({
  label,
  value,
  statusColor = 'text-white/60',
  isBoolean = false,
}: SafetySummaryItemProps) {
  const displayValue = isBoolean
    ? value
      ? '是'
      : '否'
    : String(value);

  return (
    <div className="flex items-center justify-between rounded-md bg-white/5 px-3 py-2">
      <span className="text-xs text-white/40">{label}</span>
      <span className={`text-xs font-medium ${statusColor}`}>
        {displayValue}
      </span>
    </div>
  );
}
