import type { CaseStrategy, EmptyPolicy, Separator, TokenConfig, WrapperStyle } from '../../../types';

const SEPARATORS: Array<{ value: Separator; label: string }> = [
  { value: 'Dot', label: '点 .' },
  { value: 'Space', label: '空格' },
  { value: 'Dash', label: '横杠 -' },
  { value: 'Underscore', label: '下划线 _' },
  { value: 'None', label: '无' },
];

const CASE_STRATEGIES: Array<{ value: CaseStrategy; label: string }> = [
  { value: 'AsIs', label: '原样' },
  { value: 'TitleCase', label: '标题大小写' },
  { value: 'LowerCase', label: '小写' },
  { value: 'UpperCase', label: '大写' },
  { value: 'PtDotStyle', label: 'PT 点号' },
];

const WRAPPERS: Array<{ value: WrapperStyle; label: string }> = [
  { value: 'None', label: '无' },
  { value: 'Parentheses', label: '( )' },
  { value: 'Brackets', label: '[ ]' },
  { value: 'Braces', label: '{ }' },
];

const EMPTY_POLICIES: Array<{ value: EmptyPolicy; label: string }> = [
  { value: 'Hide', label: '空值隐藏' },
  { value: 'Default', label: '默认' },
  { value: 'NeedsReview', label: '需复查' },
];

interface NamingTokenConfigEditorProps {
  token: TokenConfig;
  onChange: (token: TokenConfig) => void;
}

export function NamingTokenConfigEditor({ token, onChange }: NamingTokenConfigEditorProps) {
  const update = (patch: Partial<TokenConfig>) => onChange({ ...token, ...patch });

  return (
    <div className="mt-1 grid grid-cols-2 gap-1.5 rounded-lg border border-border/60 bg-surface/60 p-2">
      <label className="flex items-center gap-1.5 text-xs text-text-secondary">
        <input
          type="checkbox"
          className="rounded border-border"
          checked={token.enabled}
          onChange={(event) => update({ enabled: event.target.checked })}
        />
        启用
      </label>

      <SelectField
        label="分隔符"
        value={token.separator}
        options={SEPARATORS}
        onChange={(value) => update({ separator: value as Separator })}
      />

      <InputField label="前缀" value={token.prefix} onChange={(value) => update({ prefix: value })} />
      <InputField label="后缀" value={token.suffix} onChange={(value) => update({ suffix: value })} />

      <SelectField
        label="大小写"
        value={token.case_strategy}
        options={CASE_STRATEGIES}
        onChange={(value) => update({ case_strategy: value as CaseStrategy })}
      />

      <SelectField
        label="包裹"
        value={token.wrapper}
        options={WRAPPERS}
        onChange={(value) => update({ wrapper: value as WrapperStyle })}
      />

      <SelectField
        label="空值"
        value={token.empty_policy}
        options={EMPTY_POLICIES}
        onChange={(value) => update({ empty_policy: value as EmptyPolicy })}
      />
    </div>
  );
}

function InputField({ label, value, onChange }: { label: string; value: string; onChange: (value: string) => void }) {
  return (
    <label className="text-xs text-text-secondary">
      <span className="mb-0.5 block">{label}</span>
      <input
        className="w-full rounded border border-border bg-background px-1.5 py-1 text-xs text-text-primary"
        value={value}
        onChange={(event) => onChange(event.target.value)}
      />
    </label>
  );
}

function SelectField<T extends string>({
  label,
  value,
  options,
  onChange,
}: {
  label: string;
  value: T;
  options: Array<{ value: T; label: string }>;
  onChange: (value: T) => void;
}) {
  return (
    <label className="text-xs text-text-secondary">
      <span className="mb-0.5 block">{label}</span>
      <select
        className="w-full rounded border border-border bg-background px-1.5 py-1 text-xs text-text-primary"
        value={value}
        onChange={(event) => onChange(event.target.value as T)}
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </label>
  );
}
