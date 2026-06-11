import type { ComponentProps } from 'react';
import { TmdbInspector } from './TmdbInspector';

type TmdbInspectorSidebarProps = ComponentProps<typeof TmdbInspector>;

export function TmdbInspectorSidebar(props: TmdbInspectorSidebarProps) {
  return (
    <div className="flex w-[320px] shrink-0 flex-col gap-3 overflow-y-auto">
      <TmdbInspector {...props} />
    </div>
  );
}
