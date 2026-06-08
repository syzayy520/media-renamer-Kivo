// features/audit/state/auditPageStore.ts — 审计页面状态
// 职责：管理任务列表、选中任务、审计日志、加载和错误状态

import { create } from 'zustand';
import type { RenameTask, AuditLogEntry } from '../../../api/audit/types';

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
}));
