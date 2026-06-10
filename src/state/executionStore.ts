import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type {
  SafeExecuteInput,
  SafeExecuteOutput,
  ExecutionMode,
  ExecutionUIState,
  ExecutionConfirmState,
  ExecutionProgressState,
  ExecutionResultState,
} from '../types';
import { usePipelineStore } from './pipelineStore';
import { useUiFeedbackStore } from './uiFeedbackStore';

interface ExecutionState {
  // UI 状态
  uiState: ExecutionUIState;
  confirmState: ExecutionConfirmState;
  progressState: ExecutionProgressState;
  resultState: ExecutionResultState;
  
  // 操作
  startExecution: (mode: ExecutionMode) => Promise<void>;
  confirmExecution: () => Promise<void>;
  cancelExecution: () => void;
  resetExecution: () => void;
}

const initialConfirmState: ExecutionConfirmState = {
  safety_report: null,
  preview_count: 0,
  has_blockers: false,
  blocking_reasons: [],
};

const initialProgressState: ExecutionProgressState = {
  total: 0,
  completed: 0,
  success: 0,
  failed: 0,
  skipped: 0,
  blocked: 0,
  current_item: null,
};

const initialResultState: ExecutionResultState = {
  output: null,
  error: null,
  has_rollback_plan: false,
  can_rollback: false,
};

export const useExecutionStore = create<ExecutionState>((set, get) => ({
  uiState: 'idle',
  confirmState: initialConfirmState,
  progressState: initialProgressState,
  resultState: initialResultState,

  startExecution: async (mode: ExecutionMode) => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);

    try {
      const pipelineResult = usePipelineStore.getState().pipelineResult;
      if (!pipelineResult) {
        throw new Error('No pipeline result available');
      }

      // 更新确认状态
      const confirmState: ExecutionConfirmState = {
        safety_report: pipelineResult.safety,
        preview_count: pipelineResult.previews.length,
        has_blockers: !pipelineResult.safety.can_execute,
        blocking_reasons: pipelineResult.safety.blocking_reasons,
      };

      set({
        uiState: 'confirming',
        confirmState,
      });

      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      set({ uiState: 'failed' });
      setLoading(false);
    }
  },

  confirmExecution: async () => {
    const { setLoading, setError } = useUiFeedbackStore.getState();
    setLoading(true);
    setError(null);

    try {
      const pipelineResult = usePipelineStore.getState().pipelineResult;
      if (!pipelineResult) {
        throw new Error('No pipeline result available');
      }

      // 准备执行输入
      const input: SafeExecuteInput = {
        task_id: pipelineResult.task_id,
        preview_items: pipelineResult.previews,
        mode: 'Confirmed',
        user_confirmed: true,
      };

      // 更新进度状态
      set({
        uiState: 'executing',
        progressState: {
          total: pipelineResult.previews.length,
          completed: 0,
          success: 0,
          failed: 0,
          skipped: 0,
          blocked: 0,
          current_item: null,
        },
      });

      // 调用后端执行命令
      const output = await invoke<SafeExecuteOutput>('safe_execute_rename', { input });

      // 更新结果状态
      const resultState: ExecutionResultState = {
        output,
        error: output.rejection_reason || null,
        has_rollback_plan: output.rollback_plan !== null,
        can_rollback: output.rollback_plan !== null && output.rollback_plan.entries.some(e => e.executed),
      };

      set({
        uiState: 'completed',
        resultState,
        progressState: {
          total: output.summary?.total || 0,
          completed: (output.summary?.success || 0) + (output.summary?.failed || 0) + (output.summary?.blocked || 0),
          success: output.summary?.success || 0,
          failed: output.summary?.failed || 0,
          skipped: output.summary?.skipped || 0,
          blocked: output.summary?.blocked || 0,
          current_item: null,
        },
      });

      setLoading(false);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      set({ uiState: 'failed' });
      setLoading(false);
    }
  },

  cancelExecution: () => {
    set({
      uiState: 'idle',
      confirmState: initialConfirmState,
      progressState: initialProgressState,
      resultState: initialResultState,
    });
  },

  resetExecution: () => {
    set({
      uiState: 'idle',
      confirmState: initialConfirmState,
      progressState: initialProgressState,
      resultState: initialResultState,
    });
  },
}));