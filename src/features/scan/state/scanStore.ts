// features/scan/state/scanStore.ts — 扫描状态管理
// 职责：管理扫描目录路径、加载状态、PipelineResult、错误

import { create } from 'zustand';
import type { PipelineResult } from '../../../api/session/types';

interface ScanState {
  // 输入
  directory: string;
  setDirectory: (path: string) => void;

  // 状态
  isScanning: boolean;
  error: string | null;
  result: PipelineResult | null;

  // 操作
  setScanning: (loading: boolean) => void;
  setResult: (result: PipelineResult) => void;
  setError: (error: string) => void;
  clearResult: () => void;
  reset: () => void;
}

export const useScanStore = create<ScanState>((set) => ({
  directory: '',
  setDirectory: (path) => set({ directory: path, error: null }),

  isScanning: false,
  error: null,
  result: null,

  setScanning: (loading) => set({ isScanning: loading }),
  setResult: (result) => set({ result, error: null, isScanning: false }),
  setError: (error) => set({ error, isScanning: false }),
  clearResult: () => set({ result: null, error: null, isScanning: true }),
  reset: () => set({ directory: '', result: null, error: null, isScanning: false }),
}));
