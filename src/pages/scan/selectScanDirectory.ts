import { invoke } from '@tauri-apps/api/core';

export async function selectScanDirectory(): Promise<string | null> {
  const selected = await invoke<string | string[] | null>('plugin:dialog|open', {
    options: {
      title: '选择扫描目录',
      directory: true,
      multiple: false,
    },
  });

  if (Array.isArray(selected)) {
    return selected[0] ?? null;
  }

  return selected;
}
