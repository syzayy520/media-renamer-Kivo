import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { RenameTask } from '../types';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface TaskState {
  tasks: RenameTask[];
  currentTask: RenameTask | null;
  fetchTask: (taskId: string) => Promise<void>;
  fetchAllTasks: () => Promise<void>;
}

export const useTaskStore = create<TaskState>((set) => ({
  tasks: [],
  currentTask: null,

  fetchTask: async (taskId: string) => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const task = await invoke<RenameTask | null>('get_task', { taskId });
      set({ currentTask: task });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },

  fetchAllTasks: async () => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const tasks = await invoke<RenameTask[]>('get_all_tasks');
      set({ tasks });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },
}));
