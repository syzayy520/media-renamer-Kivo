# UI 状态模型

> 阶段 3：系统架构设计
> 项目：media-renamer-Kivo
> 日期：2026-06-08

---

## 一、状态管理架构

```
┌─────────────────────────────────────────────────┐
│                  UI Components                   │
│  pages/  flows/  components/                     │
│     │         │         │                        │
│     └────────┬┘─────────┘                        │
│              │ useStore()                        │
│              ↓                                   │
│  ┌──────────────────────────────────────────┐   │
│  │            Zustand Stores                │   │
│  │  scanStore | previewStore | executionStore│   │
│  │  rollbackStore | configStore              │   │
│  └──────────────────────────────────────────┘   │
│              │                                   │
│              │ invoke() / listen()               │
│              ↓                                   │
│  ┌──────────────────────────────────────────┐   │
│  │         Tauri IPC Bridge                 │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

---

## 二、Store 清单

| Store | 文件路径 | 职责 |
|-------|----------|------|
| scanStore | state/scan-state/index.ts | 扫描状态、进度、结果 |
| previewStore | state/preview-state/index.ts | 预览列表、筛选、排序 |
| executionStore | state/execution-state/index.ts | 执行状态、进度、结果 |
| rollbackStore | state/rollback-state/index.ts | 回滚状态、结果 |
| configStore | state/config-state/index.ts | 配置、模板、阈值 |
| metadataStore | state/metadata-state/index.ts | TMDb API Key 管理、连接状态 |

---

## 三、Store 详细定义

### 3.1 scanStore

```typescript
// state/scan-state/index.ts

interface ScanState {
  // 状态
  status: 'idle' | 'scanning' | 'completed' | 'error';
  scanPath: string | null;

  // 进度
  progress: {
    scanned: number;
    total: number;
    currentFile: string;
  };

  // 结果
  report: ScanReport | null;
  mediaItems: MediaItem[];

  // 错误
  error: string | null;
  permissionErrors: string[];
}

interface ScanActions {
  // 操作
  startScan: (path: string) => Promise<void>;
  cancelScan: () => void;
  reset: () => void;

  // 内部更新（由事件监听调用）
  updateProgress: (progress: ScanProgress) => void;
  setReport: (report: ScanReport) => void;
  setMediaItems: (items: MediaItem[]) => void;
  setError: (error: string) => void;
}

type ScanStore = ScanState & ScanActions;

const useScanStore = create<ScanStore>((set, get) => ({
  // 初始状态
  status: 'idle',
  scanPath: null,
  progress: { scanned: 0, total: 0, currentFile: '' },
  report: null,
  mediaItems: [],
  error: null,
  permissionErrors: [],

  // 操作
  startScan: async (path: string) => {
    set({ status: 'scanning', scanPath: path, error: null });

    // 监听进度事件
    const unlisten = await listen<ScanProgressEvent>('scan-progress', (event) => {
      get().updateProgress(event.payload);
    });

    try {
      const report = await invoke<ScanReport>('scan_directory', { path });
      const items = await invoke<MediaItem[]>('get_scanned_items');

      set({
        status: 'completed',
        report,
        mediaItems: items,
        permissionErrors: report.permissionErrors,
      });
    } catch (error) {
      set({ status: 'error', error: String(error) });
    } finally {
      unlisten();
    }
  },

  cancelScan: () => {
    invoke('cancel_scan');
    set({ status: 'idle' });
  },

  reset: () => set({
    status: 'idle',
    scanPath: null,
    progress: { scanned: 0, total: 0, currentFile: '' },
    report: null,
    mediaItems: [],
    error: null,
    permissionErrors: [],
  }),

  updateProgress: (progress) => set({ progress }),
  setReport: (report) => set({ report }),
  setMediaItems: (items) => set({ mediaItems: items }),
  setError: (error) => set({ status: 'error', error }),
}));
```

---

### 3.2 previewStore

```typescript
// state/preview-state/index.ts

interface PreviewState {
  // 状态
  status: 'idle' | 'loading' | 'ready' | 'error';

  // 数据
  previewItems: RenamePreviewItem[];
  filteredItems: RenamePreviewItem[];

  // 筛选
  filters: {
    mediaType: MediaType | 'all';
    status: 'all' | 'needsReview' | 'conflict' | 'ready';
    searchQuery: string;
  };

  // 排序
  sort: {
    field: 'fileName' | 'mediaType' | 'confidence' | 'status';
    direction: 'asc' | 'desc';
  };

  // 选中
  selectedIds: Set<string>;

  // 统计
  stats: {
    total: number;
    ready: number;
    needsReview: number;
    conflict: number;
    skipped: number;
  };

  // 错误
  error: string | null;
}

interface PreviewActions {
  // 操作
  generatePreview: (parsedItems: ParsedMediaInfo[]) => Promise<void>;
  updateItem: (id: string, updates: Partial<RenamePreviewItem>) => void;
  toggleItem: (id: string) => void;
  toggleAll: () => void;
  setFilter: (filter: Partial<PreviewState['filters']>) => void;
  setSort: (sort: PreviewState['sort']) => void;
  reset: () => void;

  // 计算
  getSelectedItems: () => RenamePreviewItem[];
  getStats: () => PreviewState['stats'];
}

const usePreviewStore = create<PreviewStore>((set, get) => ({
  // 初始状态
  status: 'idle',
  previewItems: [],
  filteredItems: [],
  filters: {
    mediaType: 'all',
    status: 'all',
    searchQuery: '',
  },
  sort: {
    field: 'status',
    direction: 'asc',
  },
  selectedIds: new Set(),
  stats: { total: 0, ready: 0, needsReview: 0, conflict: 0, skipped: 0 },
  error: null,

  // 操作
  generatePreview: async (parsedItems) => {
    set({ status: 'loading' });

    try {
      const items = await invoke<RenamePreviewItem[]>('generate_preview', {
        parsedItems,
      });

      const stats = calculateStats(items);

      set({
        status: 'ready',
        previewItems: items,
        stats,
      });

      // 自动应用筛选
      get().applyFilters();
    } catch (error) {
      set({ status: 'error', error: String(error) });
    }
  },

  updateItem: (id, updates) => {
    set((state) => ({
      previewItems: state.previewItems.map((item) =>
        item.id === id ? { ...item, ...updates } : item
      ),
    }));
    get().applyFilters();
  },

  toggleItem: (id) => {
    set((state) => {
      const newSelected = new Set(state.selectedIds);
      if (newSelected.has(id)) {
        newSelected.delete(id);
      } else {
        newSelected.add(id);
      }
      return { selectedIds: newSelected };
    });
  },

  toggleAll: () => {
    set((state) => {
      const allIds = state.filteredItems.map((item) => item.id);
      const allSelected = allIds.every((id) => state.selectedIds.has(id));

      return {
        selectedIds: allSelected ? new Set() : new Set(allIds),
      };
    });
  },

  setFilter: (filter) => {
    set((state) => ({
      filters: { ...state.filters, ...filter },
    }));
    get().applyFilters();
  },

  setSort: (sort) => {
    set({ sort });
    get().applyFilters();
  },

  reset: () => set({
    status: 'idle',
    previewItems: [],
    filteredItems: [],
    filters: { mediaType: 'all', status: 'all', searchQuery: '' },
    sort: { field: 'status', direction: 'asc' },
    selectedIds: new Set(),
    stats: { total: 0, ready: 0, needsReview: 0, conflict: 0, skipped: 0 },
    error: null,
  }),

  getSelectedItems: () => {
    const { previewItems, selectedIds } = get();
    return previewItems.filter((item) => selectedIds.has(item.id));
  },

  getStats: () => get().stats,

  // 内部方法
  applyFilters: () => {
    const { previewItems, filters, sort } = get();

    let filtered = [...previewItems];

    // 应用筛选
    if (filters.mediaType !== 'all') {
      filtered = filtered.filter((item) => item.parsedInfo.mediaType === filters.mediaType);
    }

    if (filters.status !== 'all') {
      switch (filters.status) {
        case 'needsReview':
          filtered = filtered.filter((item) => item.needsReview);
          break;
        case 'conflict':
          filtered = filtered.filter((item) => item.conflicts.length > 0);
          break;
        case 'ready':
          filtered = filtered.filter((item) => !item.needsReview && item.conflicts.length === 0);
          break;
      }
    }

    if (filters.searchQuery) {
      const query = filters.searchQuery.toLowerCase();
      filtered = filtered.filter((item) =>
        item.originalPath.toLowerCase().includes(query) ||
        item.newName.toLowerCase().includes(query)
      );
    }

    // 应用排序
    filtered.sort((a, b) => {
      let comparison = 0;

      switch (sort.field) {
        case 'fileName':
          comparison = a.originalPath.localeCompare(b.originalPath);
          break;
        case 'mediaType':
          comparison = a.parsedInfo.mediaType.localeCompare(b.parsedInfo.mediaType);
          break;
        case 'confidence':
          comparison = a.parsedInfo.confidence - b.parsedInfo.confidence;
          break;
        case 'status':
          // 冲突 > 低置信度 > 就绪
          const statusA = a.conflicts.length > 0 ? 2 : a.needsReview ? 1 : 0;
          const statusB = b.conflicts.length > 0 ? 2 : b.needsReview ? 1 : 0;
          comparison = statusA - statusB;
          break;
      }

      return sort.direction === 'asc' ? comparison : -comparison;
    });

    set({ filteredItems: filtered });
  },
}));
```

---

### 3.3 executionStore

```typescript
// state/execution-state/index.ts

interface ExecutionState {
  // 状态
  status: 'idle' | 'confirming' | 'executing' | 'completed' | 'error';

  // 任务
  task: RenameTask | null;

  // 进度
  progress: {
    completed: number;
    total: number;
    currentFile: string;
    currentStatus: 'renaming' | 'success' | 'failed' | 'skipped';
  };

  // 结果
  results: RenameResult[];
  stats: {
    renamed: number;
    skipped: number;
    failed: number;
  };

  // 错误
  error: string | null;
}

interface ExecutionActions {
  // 操作
  confirmExecution: (items: RenamePreviewItem[]) => void;
  execute: (items: RenamePreviewItem[], dryRun: boolean) => Promise<void>;
  reset: () => void;

  // 内部更新
  updateProgress: (progress: RenameProgressEvent) => void;
  setTask: (task: RenameTask) => void;
  setError: (error: string) => void;
}

const useExecutionStore = create<ExecutionStore>((set, get) => ({
  // 初始状态
  status: 'idle',
  task: null,
  progress: { completed: 0, total: 0, currentFile: '', currentStatus: 'renaming' },
  results: [],
  stats: { renamed: 0, skipped: 0, failed: 0 },
  error: null,

  // 操作
  confirmExecution: (items) => {
    set({
      status: 'confirming',
      progress: { completed: 0, total: items.length, currentFile: '', currentStatus: 'renaming' },
    });
  },

  execute: async (items, dryRun) => {
    set({ status: 'executing', error: null });

    // 监听进度事件
    const unlisten = await listen<RenameProgressEvent>('rename-progress', (event) => {
      get().updateProgress(event.payload);
    });

    try {
      const task = await invoke<RenameTask>('execute_rename', {
        previewItems: items,
        dryRun,
      });

      const stats = {
        renamed: task.renamedCount,
        skipped: task.skippedCount,
        failed: task.failedCount,
      };

      set({
        status: 'completed',
        task,
        results: task.results,
        stats,
      });
    } catch (error) {
      set({ status: 'error', error: String(error) });
    } finally {
      unlisten();
    }
  },

  reset: () => set({
    status: 'idle',
    task: null,
    progress: { completed: 0, total: 0, currentFile: '', currentStatus: 'renaming' },
    results: [],
    stats: { renamed: 0, skipped: 0, failed: 0 },
    error: null,
  }),

  updateProgress: (progress) => set({ progress }),
  setTask: (task) => set({ task }),
  setError: (error) => set({ status: 'error', error }),
}));
```

---

### 3.4 rollbackStore

```typescript
// state/rollback-state/index.ts

interface RollbackState {
  // 状态
  status: 'idle' | 'confirming' | 'rolling-back' | 'completed' | 'error';

  // 数据
  taskHistory: RenameTask[];
  selectedTask: RenameTask | null;
  rollbackRecord: RollbackRecord | null;

  // 进度
  progress: {
    completed: number;
    total: number;
    currentFile: string;
    currentStatus: 'rolling-back' | 'success' | 'failed';
  };

  // 错误
  error: string | null;
}

interface RollbackActions {
  // 操作
  loadHistory: () => Promise<void>;
  selectTask: (taskId: string) => void;
  confirmRollback: () => void;
  executeRollback: (taskId: string) => Promise<void>;
  reset: () => void;

  // 内部更新
  updateProgress: (progress: RollbackProgressEvent) => void;
  setRollbackRecord: (record: RollbackRecord) => void;
  setError: (error: string) => void;
}

const useRollbackStore = create<RollbackStore>((set, get) => ({
  // 初始状态
  status: 'idle',
  taskHistory: [],
  selectedTask: null,
  rollbackRecord: null,
  progress: { completed: 0, total: 0, currentFile: '', currentStatus: 'rolling-back' },
  error: null,

  // 操作
  loadHistory: async () => {
    try {
      const history = await invoke<RenameTask[]>('get_task_history');
      set({ taskHistory: history });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  selectTask: (taskId) => {
    const { taskHistory } = get();
    const task = taskHistory.find((t) => t.id === taskId);
    set({ selectedTask: task || null });
  },

  confirmRollback: () => {
    set({ status: 'confirming' });
  },

  executeRollback: async (taskId) => {
    set({ status: 'rolling-back', error: null });

    // 监听进度事件
    const unlisten = await listen<RollbackProgressEvent>('rollback-progress', (event) => {
      get().updateProgress(event.payload);
    });

    try {
      const record = await invoke<RollbackRecord>('rollback_task', { taskId });

      set({
        status: 'completed',
        rollbackRecord: record,
      });

      // 刷新历史
      get().loadHistory();
    } catch (error) {
      set({ status: 'error', error: String(error) });
    } finally {
      unlisten();
    }
  },

  reset: () => set({
    status: 'idle',
    taskHistory: [],
    selectedTask: null,
    rollbackRecord: null,
    progress: { completed: 0, total: 0, currentFile: '', currentStatus: 'rolling-back' },
    error: null,
  }),

  updateProgress: (progress) => set({ progress }),
  setRollbackRecord: (record) => set({ rollbackRecord: record }),
  setError: (error) => set({ status: 'error', error }),
}));
```

---

### 3.5 configStore

```typescript
// state/config-state/index.ts

interface ConfigState {
  // 配置
  config: AppConfig | null;

  // 模板
  templates: RenameRule[];

  // 阈值
  confidenceThreshold: number;

  // 状态
  status: 'idle' | 'loading' | 'ready' | 'error';
  error: string | null;
}

interface ConfigActions {
  // 操作
  loadConfig: () => Promise<void>;
  updateTemplate: (mediaType: MediaType, template: string) => Promise<void>;
  updateThreshold: (value: number) => Promise<void>;
  reset: () => void;
}

const useConfigStore = create<ConfigStore>((set, get) => ({
  // 初始状态
  config: null,
  templates: [],
  confidenceThreshold: 70,
  status: 'idle',
  error: null,

  // 操作
  loadConfig: async () => {
    set({ status: 'loading' });

    try {
      const [config, templates] = await Promise.all([
        invoke<AppConfig>('get_config'),
        invoke<RenameRule[]>('get_templates'),
      ]);

      set({
        status: 'ready',
        config,
        templates,
        confidenceThreshold: config.thresholds.confidence,
      });
    } catch (error) {
      set({ status: 'error', error: String(error) });
    }
  },

  updateTemplate: async (mediaType, template) => {
    try {
      await invoke('update_template', { mediaType, template });

      set((state) => ({
        templates: state.templates.map((t) =>
          t.mediaType === mediaType ? { ...t, template } : t
        ),
      }));
    } catch (error) {
      set({ error: String(error) });
    }
  },

  updateThreshold: async (value) => {
    try {
      await invoke('update_confidence_threshold', { value });
      set({ confidenceThreshold: value });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  reset: () => set({
    config: null,
    templates: [],
    confidenceThreshold: 70,
    status: 'idle',
    error: null,
  }),
}));
```

---

### 3.6 metadataStore

```typescript
// state/metadata-state/index.ts

interface MetadataState {
  // TMDb API Key
  tmdbApiKey: string | null;
  maskedApiKey: string | null; // 遮罩显示

  // 连接状态
  connectionStatus: 'idle' | 'testing' | 'connected' | 'error';
  connectionError: string | null;

  // 配置状态
  isConfigured: boolean;

  // 状态
  status: 'idle' | 'loading' | 'ready' | 'error';
  error: string | null;
}

interface MetadataActions {
  // 操作
  loadApiKey: () => Promise<void>;
  saveApiKey: (apiKey: string) => Promise<void>;
  clearApiKey: () => Promise<void>;
  testConnection: () => Promise<void>;
  reset: () => void;

  // 内部更新
  setApiKey: (apiKey: string | null) => void;
  setMaskedApiKey: (masked: string | null) => void;
  setConnectionStatus: (status: MetadataState['connectionStatus']) => void;
  setError: (error: string) => void;
}

type MetadataStore = MetadataState & MetadataActions;

const useMetadataStore = create<MetadataStore>((set, get) => ({
  // 初始状态
  tmdbApiKey: null,
  maskedApiKey: null,
  connectionStatus: 'idle',
  connectionError: null,
  isConfigured: false,
  status: 'idle',
  error: null,

  // 操作
  loadApiKey: async () => {
    set({ status: 'loading' });

    try {
      const result = await invoke<{ apiKey: string | null; maskedApiKey: string | null }>('get_tmdb_api_key');

      set({
        status: 'ready',
        tmdbApiKey: result.apiKey,
        maskedApiKey: result.maskedApiKey,
        isConfigured: result.apiKey !== null,
      });
    } catch (error) {
      set({ status: 'error', error: String(error) });
    }
  },

  saveApiKey: async (apiKey: string) => {
    set({ status: 'loading' });

    try {
      const result = await invoke<{ maskedApiKey: string }>('save_tmdb_api_key', { apiKey });

      set({
        status: 'ready',
        tmdbApiKey: apiKey,
        maskedApiKey: result.maskedApiKey,
        isConfigured: true,
        connectionStatus: 'idle',
        connectionError: null,
      });
    } catch (error) {
      set({ status: 'error', error: String(error) });
    }
  },

  clearApiKey: async () => {
    set({ status: 'loading' });

    try {
      await invoke('clear_tmdb_api_key');

      set({
        status: 'ready',
        tmdbApiKey: null,
        maskedApiKey: null,
        isConfigured: false,
        connectionStatus: 'idle',
        connectionError: null,
      });
    } catch (error) {
      set({ status: 'error', error: String(error) });
    }
  },

  testConnection: async () => {
    set({ connectionStatus: 'testing', connectionError: null });

    try {
      await invoke('test_tmdb_connection');

      set({
        connectionStatus: 'connected',
        connectionError: null,
      });
    } catch (error) {
      set({
        connectionStatus: 'error',
        connectionError: String(error),
      });
    }
  },

  reset: () => set({
    tmdbApiKey: null,
    maskedApiKey: null,
    connectionStatus: 'idle',
    connectionError: null,
    isConfigured: false,
    status: 'idle',
    error: null,
  }),

  // 内部更新
  setApiKey: (apiKey) => set({ tmdbApiKey: apiKey }),
  setMaskedApiKey: (masked) => set({ maskedApiKey: masked }),
  setConnectionStatus: (status) => set({ connectionStatus: status }),
  setError: (error) => set({ status: 'error', error }),
}));
```

---

## 四、Store 间通信

### 4.1 流程串联

```
scanStore.completed
    ↓ 触发
previewStore.generatePreview(scanStore.mediaItems)
    ↓ 用户确认
executionStore.confirmExecution(previewStore.getSelectedItems())
    ↓ 用户点击执行
executionStore.execute(items)
    ↓ 完成后
rollbackStore.loadHistory()  // 刷新历史
```

### 4.2 实现方式

```typescript
// 在 flows/scan-flow/index.ts 中串联
export function useScanFlow() {
  const scanStore = useScanStore();
  const previewStore = usePreviewStore();

  const handleScanComplete = async () => {
    if (scanStore.status === 'completed' && scanStore.mediaItems.length > 0) {
      // 解析
      const parsed = await invoke<ParsedMediaInfo[]>('parse_media_files', {
        mediaItems: scanStore.mediaItems,
      });

      // 生成预览
      await previewStore.generatePreview(parsed);
    }
  };

  return { handleScanComplete };
}
```

---

## 五、状态持久化

### 5.1 策略

| 状态 | 持久化 | 原因 |
|------|--------|------|
| scanStore | 否 | 扫描结果应重新获取 |
| previewStore | 否 | 预览应重新生成 |
| executionStore | 否 | 执行状态是临时的 |
| rollbackStore | 否 | 历史从 DB 加载 |
| configStore | 是 (DB) | 配置需要持久化 |
| metadataStore | 是 (DB) | TMDb API Key 需要安全持久化 |

### 5.2 应用重启恢复

```typescript
// 应用启动时
export function useAppInit() {
  const configStore = useConfigStore();
  const metadataStore = useMetadataStore();

  useEffect(() => {
    configStore.loadConfig();
    metadataStore.loadApiKey();
  }, []);
}
```

---

*下一阶段：ui-review-notes.md*
