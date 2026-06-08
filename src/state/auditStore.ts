import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AuditLogEntry } from '../types';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface AuditState {
  auditLogs: AuditLogEntry[];
  fetchAuditLogs: (taskId: string) => Promise<void>;
}

export const useAuditStore = create<AuditState>((set) => ({
  auditLogs: [],

  fetchAuditLogs: async (taskId: string) => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);
    try {
      const logs = await invoke<AuditLogEntry[]>('get_audit_logs', { taskId });
      set({ auditLogs: logs });
      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      setLoading(false);
    }
  },
}));
