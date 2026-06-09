# P2-009 — TMDb Search Design Spike

## 1. 目标与非目标

### 目标
- 为 Preview Workbench v2 的未来 TMDb 搜索能力做设计尖刺
- 明确后续如何安全接入 TMDb 搜索、候选选择、速率限制、错误状态和 API Key 边界
- 为后续实现提供清晰的设计蓝图和拆票依据

### 非目标（本票不实现）
- 不实现真实 TMDb 搜索
- 不联网调用 TMDb API
- 不读取 API key 明文
- 不调用 backend command
- 不修改候选状态 store
- 不开放 rename / rollback / export
- 不修改任何前端组件或后端代码

## 2. 当前状态盘点

### 已有基础
1. **TMDb API Key 设置页**
   - 已支持保存/清除 API key
   - 只返回 configured 状态（不返回 key 明文）
   - 安全边界：前端无法读取 key 明文

2. **Preview Workbench v2 已有组件**
   - File Queue Panel（文件队列）
   - Candidate Panel（候选面板，当前为静态/disabled scaffold）
   - Rename Preview Panel（重命名预览面板）
   - Dry-run Action Bar（干运行操作栏）
   - Selection Store（选择状态管理）
   - Safety Summary（安全摘要）

3. **状态管理**
   - workbenchSelectionStore 只保存选择 ID，不保存完整数据对象
   - scanStore 只读访问，不修改

### 当前限制
- Candidate Panel 仍是静态/disabled scaffold
- 没有 TMDb 搜索能力
- 没有候选选择机制
- 没有速率限制或错误处理

## 3. 安全边界

### API Key 安全
- **前端不得读取 API key 明文**
- **未来真实 TMDb 请求优先走后端 command**
- **后端 command 也只能读取本地安全存储，不向前端回传 key**
- **不在日志、错误、测试快照里输出 key**
- **真实搜索必须有 rate limit / timeout / error handling**

### 搜索结果安全
- **搜索结果只作为候选，不自动应用**
- **用户必须显式选择候选**
- **即使选择候选，真实 rename 仍不得开放**

### 数据安全
- 不保存完整大型 TMDb 原始响应
- 只保存必要的候选 DTO
- 搜索状态不得塞进 selection store

## 4. 未来后端命令设计草案

### 建议命令名称
```
search_tmdb_candidates
```

### 输入草案
```typescript
interface SearchTmdbCandidatesInput {
  /** 搜索查询字符串 */
  query: string;
  /** 媒体类型：电影或电视剧 */
  media_type: 'Movie' | 'Tv';
  /** 结果语言（ISO 639-1） */
  language: string;
  /** 年份过滤（可选） */
  year?: number;
  /** 分页页码（可选，默认 1） */
  page?: number;
}
```

### 输出草案

#### SearchTmdbCandidatesOutput
```typescript
interface SearchTmdbCandidatesOutput {
  /** 候选列表 */
  candidates: TmdbCandidate[];
  /** 数据来源标识，固定为 'tmdb' */
  source: 'tmdb';
  /** 速率限制状态（如果可用） */
  rate_limit?: RateLimitState;
  /** 结构化错误（如果有） */
  error?: TmdbSearchError;
}
```

#### TmdbCandidate
```typescript
interface TmdbCandidate {
  /** 前端候选 ID，不等于 TMDb 原始 ID */
  id: string;
  /** TMDb ID */
  tmdb_id: number;
  /** 标题（电影名/剧集名） */
  title: string;
  /** 原始标题（可能为外语，可选） */
  original_title?: string;
  /** 媒体类型：'Movie' 或 'Tv' */
  media_type: 'Movie' | 'Tv';
  /** 发布年份（可选） */
  release_year?: number;
  /** 概述（截断至 500 字符，可选） */
  overview?: string;
  /** 海报路径（相对路径，可选） */
  poster_path?: string;
  /** 背景图路径（相对路径，可选） */
  backdrop_path?: string;
  /** 语言（ISO 639-1，可选） */
  language?: string;
  /** 热度评分（可选） */
  popularity?: number;
  /** TMDb 评分（0-10，可选） */
  vote_average?: number;
  /** 匹配置信度（0-1，由后端计算，可选） */
  confidence_hint?: number;
  /** 匹配原因列表（只保存必要解释，不保存大对象） */
  match_reasons: string[];
}
```

**设计说明：**
- `id` 是前端候选 ID，不等于 TMDb 原始 ID，用于前端状态管理
- `tmdb_id` 是 TMDb 数据库中的唯一标识符
- 不透传完整 TMDb 原始 JSON，只提取必要字段
- `match_reasons` 只保存必要解释，不保存大对象，例如 `['title_match', 'year_match']`
- 所有字段都是可选的，除了 `id`、`tmdb_id`、`title`、`media_type`、`match_reasons`

#### RateLimitState
```typescript
interface RateLimitState {
  /** 剩余请求次数 */
  remaining: number;
  /** 重置时间（Unix 时间戳，秒） */
  reset_at: number;
  /** 总请求限制（可选） */
  limit?: number;
}
```

#### TmdbSearchError
```typescript
interface TmdbSearchError {
  /** 错误代码 */
  code: 'API_KEY_MISSING' | 'API_KEY_INVALID' | 'RATE_LIMITED' | 'TIMEOUT' | 'NETWORK_ERROR' | 'UNKNOWN';
  /** 人类可读错误消息 */
  message: string;
  /** 是否可重试 */
  retryable: boolean;
  /** 重试等待时间（秒，如果可重试） */
  retry_after?: number;
}
```

#### 示例响应
```typescript
// 正常结果示例
const successExample: SearchTmdbCandidatesOutput = {
  source: 'tmdb',
  candidates: [
    {
      id: 'tmdb-movie-12345',
      tmdb_id: 12345,
      title: 'Example Movie',
      media_type: 'Movie',
      release_year: 2024,
      language: 'zh-CN',
      confidence_hint: 0.86,
      match_reasons: ['title_match', 'year_match'],
    },
  ],
  rate_limit: {
    remaining: 39,
    reset_at: 1710000000,
    limit: 40,
  },
};

// 空结果示例（空结果不是异常）
const emptyExample: SearchTmdbCandidatesOutput = {
  source: 'tmdb',
  candidates: [],
};

// 错误响应示例（error 只在失败状态出现）
const errorExample: SearchTmdbCandidatesOutput = {
  source: 'tmdb',
  candidates: [],
  error: {
    code: 'RATE_LIMITED',
    message: 'API rate limit exceeded',
    retryable: true,
    retry_after: 10,
  },
};
```

**设计说明：**
- 空结果不是异常，是正常状态
- `error` 只在失败状态出现，成功时不返回
- 不返回 API key
- 不返回原始未清洗的大 JSON

### 设计原则
- **不返回 API key**
- **不返回原始未清洗的大 JSON**
- **不把 TMDb 响应直接透传给前端**
- **错误必须结构化**
- **空结果必须是正常状态，不是异常**

## 5. 前端状态设计

### 现有 Store 保持不变
- **workbenchSelectionStore 仍只保存：**
  - `selectedPreviewId`
  - `selectedCandidateId`
  - `appliedCandidateId`
- **TMDb 搜索状态不得塞进 selection store**

### 未来新 Store 设计
- **未来如需要，应单独建 `candidate-search` 子状态**
- **该状态未来属于 `candidate-search` 子功能族**
- **不放进 `workbenchSelectionStore`**
- **不保存 API key**
- **不保存完整 TMDb 原始响应**
- **不保存真实文件修改状态**
- **不保存 rename / rollback / export 状态**

- **状态结构草案：**
  ```typescript
  interface CandidateSearchState {
    /** 搜索查询字符串 */
    query: string;
    /** 搜索媒体类型 */
    mediaType: 'Movie' | 'Tv';
    /** 搜索语言 */
    language: string;
    /** 是否正在加载 */
    isLoading: boolean;
    /** 搜索结果候选列表 */
    candidates: TmdbCandidate[];
    /** 当前错误（如果有） */
    error: TmdbSearchError | null;
    /** 速率限制状态 */
    rateLimit: RateLimitState | null;
    /** 最后搜索的预览项 ID（用于关联搜索结果与文件） */
    lastSearchPreviewId: string | null;
    /** 最后更新时间（Unix 时间戳，毫秒） */
    lastUpdatedAt: number | null;
  }
  ```

#### 状态转换类型定义
```typescript
type CandidateSearchTransition =
  | { type: 'set_query'; query: string }
  | { type: 'start_search'; previewId: string }
  | { type: 'search_success'; candidates: TmdbCandidate[]; rateLimit?: RateLimitState }
  | { type: 'search_empty'; rateLimit?: RateLimitState }
  | { type: 'search_failed'; error: TmdbSearchError; rateLimit?: RateLimitState }
  | { type: 'clear_for_preview_change'; nextPreviewId: string };
```

**状态转换说明：**
- 切换 `selectedPreviewId` 时必须清理旧候选（通过 `clear_for_preview_change` 转换）
- 搜索失败不得清空已有 dry-run preview
- 搜索成功也不得自动 apply candidate
- apply candidate 只影响后续 dry-run preview，不执行真实 rename

#### 状态转换示例
```typescript
// 初始状态
const initialState: CandidateSearchState = {
  query: '',
  mediaType: 'Movie',
  language: 'zh-CN',
  isLoading: false,
  candidates: [],
  error: null,
  rateLimit: null,
  lastSearchPreviewId: null,
  lastUpdatedAt: null,
};

// 设置查询
const queryState: CandidateSearchState = {
  ...initialState,
  query: 'Fight Club',
};

// 开始搜索
const searchingState: CandidateSearchState = {
  ...queryState,
  isLoading: true,
  lastSearchPreviewId: 'preview-123',
  lastUpdatedAt: Date.now(),
};

// 搜索成功
const successState: CandidateSearchState = {
  ...searchingState,
  isLoading: false,
  candidates: [
    {
      id: 'tmdb-movie-12345',
      tmdb_id: 12345,
      title: 'Fight Club',
      media_type: 'Movie',
      release_year: 1999,
      confidence_hint: 0.95,
      match_reasons: ['title_match', 'year_match'],
    },
  ],
  rateLimit: { remaining: 39, reset_at: 1710000000, limit: 40 },
};

// 搜索空结果
const emptyState: CandidateSearchState = {
  ...searchingState,
  isLoading: false,
  candidates: [],
  rateLimit: { remaining: 38, reset_at: 1710000000, limit: 40 },
};

// 搜索失败
const errorState: CandidateSearchState = {
  ...searchingState,
  isLoading: false,
  error: {
    code: 'RATE_LIMITED',
    message: 'API rate limit exceeded',
    retryable: true,
    retry_after: 10,
  },
};

// 切换预览项时清理旧候选
const clearedState: CandidateSearchState = {
  ...successState,
  candidates: [],
  error: null,
  lastSearchPreviewId: 'preview-456',
  lastUpdatedAt: Date.now(),
};
```

### 数据安全
- **不保存 API key**
- **不保存完整大型 TMDb 原始响应**
- **只保存必要的候选 DTO**

## 6. 组件家谱设计

### 目录结构规划
```
src/features/preview-workbench/
├── candidate-search/           # 候选搜索功能族
│   ├── TmdbSearchPanel.tsx     # 搜索主面板
│   ├── TmdbSearchInput.tsx     # 搜索输入组件
│   ├── TmdbSearchResultList.tsx # 搜索结果列表
│   ├── TmdbSearchResultItem.tsx # 搜索结果项
│   ├── TmdbSearchEmptyState.tsx # 空状态
│   ├── TmdbSearchErrorState.tsx # 错误状态
│   └── TmdbSearchLoadingState.tsx # 加载状态
├── candidate-list/             # 候选列表功能族
├── state/                      # 状态管理
├── types/                      # 类型定义
└── index.ts                    # 模块入口（只 re-export）
```

### 家谱模式原则
- **文件夹 = 功能族**
- **子文件夹 = 子功能族**
- **单文件 = 最小职责**
- **禁止 helper/utils/manager/service/glue/facade/common/misc 桶文件**
- **index.ts 只 re-export**

## 7. UI/UX 行为设计

### 状态场景
1. **没有 API key 时**
   - 显示需要配置 API key，但不暴露 key
   - 引导用户前往设置页面
   - 搜索输入禁用

2. **有 API key 但未搜索**
   - 显示 empty state
   - 提示输入搜索关键词
   - 搜索输入启用

3. **搜索中**
   - loading state
   - 禁用搜索输入
   - 显示加载动画

4. **无结果**
   - empty result state
   - 提示修改搜索条件
   - 搜索输入启用

5. **请求失败**
   - error state
   - 显示错误信息（结构化）
   - 提供重试按钮（如果可重试）

6. **rate limit**
   - rate limit state
   - 显示剩余时间和限制信息
   - 禁用搜索直到重置

7. **用户选择候选后**
   - 只更新 candidate selection，不执行 rename
   - 高亮选中的候选
   - 更新 workbenchSelectionStore.selectedCandidateId
   - **注意：这是未来实现票中的行为，不是 P2-009 行为。P2-009 只设计，不修改 store。**

8. **候选应用必须仍是 dry-run preview**
   - 应用候选只影响预览，不执行真实重命名
   - 更新 workbenchSelectionStore.appliedCandidateId
   - 重命名预览面板显示候选应用后的效果
   - **注意：这是未来实现票中的行为，不是 P2-009 行为。P2-009 只设计，不修改 store。**

## 8. 错误与边界情况

### 错误场景
1. **API key 未配置**
   - 错误代码：`API_KEY_MISSING`
   - 引导用户前往设置页面

2. **query 为空**
   - 前端验证，不发送请求
   - 显示验证错误

3. **TMDb timeout**
   - 错误代码：`TIMEOUT`
   - 可重试
   - 建议稍后重试

4. **TMDb 429 rate limit**
   - 错误代码：`RATE_LIMITED`
   - 显示重置时间
   - 禁用搜索直到重置

5. **TMDb 401/403**
   - 错误代码：`API_KEY_INVALID`
   - 引导用户检查 API key

6. **网络不可用**
   - 错误代码：`NETWORK_ERROR`
   - 可重试
   - 检查网络连接

7. **结果语言缺失**
   - 使用原始语言结果
   - 标记语言缺失

8. **Movie/Tv 类型不匹配**
   - 显示类型不匹配警告
   - 允许用户手动选择正确类型

9. **多季剧集匹配不明确**
   - 显示季/集选择界面
   - 允许用户指定具体季/集

10. **搜索结果与当前文件置信度低**
    - 显示置信度警告
    - 允许用户手动选择

11. **用户切换文件时旧候选如何清理**
    - 切换文件时自动清理搜索结果
    - 重置搜索状态
    - 保留 API key 配置

## 9. 验收标准

### 后续实现时的验收标准
- **不泄露 API key**
- **不调用真实 rename / rollback**
- **不修改本地文件**
- **所有搜索结果只作为候选**
- **搜索失败不影响已有 dry-run preview**
- **action bar 仍 disabled，除非后续单独票解锁**
- **build / tsc / lint pass**
- **安全搜索无 invoke/fetch 泄漏，除非对应 ticket 明确允许**

### 测试要求
- 单元测试覆盖所有状态场景
- 集成测试覆盖搜索流程
- 安全测试验证 key 不泄露
- 错误测试覆盖所有错误场景

## 10. 后续拆票建议

### P2-010: Candidate Search Mock UI
- **范围**: 只做 mock UI，不联网
- **目标**: 实现搜索界面和状态展示
- **依赖**: P2-009 设计文档

### P2-011: TMDb Search Backend Contract
- **范围**: 只做后端 contract/types，不接 UI
- **目标**: 定义后端命令接口和类型
- **依赖**: P2-009 设计文档

### P2-012: TMDb Search Backend Implementation
- **范围**: 后端读取安全 key，调用 TMDb，有 timeout/rate limit/error
- **目标**: 实现真实 TMDb 搜索
- **依赖**: P2-011

### P2-013: TMDb Search Frontend Integration
- **范围**: 前端调用后端 command，不读取 key
- **目标**: 连接前端和后端搜索功能
- **依赖**: P2-010, P2-012

### P2-014: Candidate Apply Dry-run Preview
- **范围**: 候选应用只影响 dry-run preview，不执行 rename
- **目标**: 实现候选应用预览
- **依赖**: P2-013

### P2-015: Workbench Visual Acceptance Pass
- **范围**: 真实窗口 UI 验收
- **目标**: 完整功能验收
- **依赖**: P2-014

---

## 设计决策记录

### 决策 1: 搜索状态独立存储
**理由**: 避免污染现有的 workbenchSelectionStore，保持单一职责原则。

### 决策 2: 后端处理 TMDb 响应
**理由**: 避免前端直接处理大型 TMDb 响应，减少前端复杂度，提高安全性。

### 决策 3: 结构化错误处理
**理由**: 提供更好的错误处理和用户体验，便于前端显示友好错误信息。

### 决策 4: 候选应用只影响预览
**理由**: 保持安全边界，避免意外执行真实重命名，符合 dry-run 原则。

---

## 附录

### 术语表
- **TMDb**: The Movie Database，电影数据库
- **候选**: 搜索结果中可能匹配的媒体项
- **Dry-run**: 干运行，只预览不执行
- **API Key**: 应用程序接口密钥
- **Rate Limit**: 速率限制，防止滥用
- **DTO**: Data Transfer Object，数据传输对象

### 参考文献
- TMDb API 文档: https://developers.themoviedb.org/3
- Preview Workbench v2 设计文档
- 家谱模式规范