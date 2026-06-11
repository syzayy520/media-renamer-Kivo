import type { ComponentProps } from 'react';
import { PreviewNamingWorkbenchPanel } from './PreviewNamingWorkbenchPanel';

type PreviewNamingWorkbenchSidebarProps = ComponentProps<typeof PreviewNamingWorkbenchPanel>;

export function PreviewNamingWorkbenchSidebar(props: PreviewNamingWorkbenchSidebarProps) {
  return (
    <div className="flex w-[260px] shrink-0 flex-col gap-3 overflow-y-auto">
      <PreviewNamingWorkbenchPanel {...props} />
    </div>
  );
}
