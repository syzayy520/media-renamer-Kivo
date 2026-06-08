// api/shared/apiError.ts — 统一错误类型
// 职责：只定义前端 invoke 错误类型

export interface ApiError {
  message: string;
}
