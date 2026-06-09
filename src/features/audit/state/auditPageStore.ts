// features/audit/state/auditPageStore.ts — 审计页面状态
// 职责：管理任务列表、选中任务、审计日志、加载和错误状态、async actions

import { create } from 'zustand';
import type { RenameTask, AuditLogEntry } from '../../../api/audit/types';
import { getAllTasks } from '../../../api/audit/getAllTasks';
import { getTask } from '../../../api/audit/getTask';
import { getAuditLogs } from '../../../api/audit/getAuditLogs';

interface AuditPageState {
  tasks: RenameTask[];
  selectedTaskId: string | null;
  selectedTask: RenameTask | null;
  logs: AuditLogEntry[];
  isLoadingTasks: boolean;
  isLoadingLogs: boolean;
  error: string | null;

  setTasks: (tasks: RenameTask[]) => void;
  setSelectedTaskId: (id: string | null) => void;
  setSelectedTask: (task: RenameTask | null) => void;
  setLogs: (logs: AuditLogEntry[]) => void;
  setIsLoadingTasks: (loading: boolean) => void;
  setIsLoadingLogs: (loading: boolean) => void;
  setError: (error: string) => void;
  clearError: () => void;

  loadTasks: () => Promise<void>;
  selectTask: (id: string) => Promise<void>;
}

export const useAuditPageStore = create<AuditPageState>((set) => ({
  tasks: [],
  selectedTaskId: null,
  selectedTask: null,
  logs: [],
  isLoadingTasks: false,
  isLoadingLogs: false,
  error: null,

  setTasks: (tasks) => set({ tasks, isLoadingTasks: false }),
  setSelectedTaskId: (id) => set({ selectedTaskId: id }),
  setSelectedTask: (task) => set({ selectedTask: task }),
  setLogs: (logs) => set({ logs, isLoadingLogs: false }),
  setIsLoadingTasks: (loading) => set({ isLoadingTasks: loading }),
  setIsLoadingLogs: (loading) => set({ isLoadingLogs: loading }),
  setError: (error) => set({ error, isLoadingTasks: false, isLoadingLogs: false }),
  clearError: () => set({ error: null }),

  loadTasks: async () => {
    set({ error: null, isLoadingTasks: true });
    try {
      const data = await getAllTasks();
      set({ tasks: data, isLoadingTasks: false });
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      set({ error: msg, isLoadingTasks: false });
    }
  },

  selectTask: async (id: string) => {
    set({ selectedTaskId: id, error: null, isLoadingLogs: true });
    try {
      const [task, auditLogs] = await Promise.all([
        getTask(id),
        getAuditLogs(id),
      ]);
      set({ selectedTask: task, logs: auditLogs, isLoadingLogs: false });
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      set({ error: msg, isLoadingLogs: false });
    }
  },
}));
