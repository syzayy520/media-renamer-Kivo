// api/config/types.ts — Config 相关类型
// 职责：AppConfig / TemplatesConfig / ThresholdsConfig / RenameRule 等

import type { MediaType } from '../session/types';

export interface AppConfig {
  templates: TemplatesConfig;
  thresholds: ThresholdsConfig;
  limits: LimitsConfig;
}

export interface TemplatesConfig {
  movie: string;
  series: string;
  anime: string;
  special: string;
  extras: string;
}

export interface ThresholdsConfig {
  confidence: number;
}

export interface LimitsConfig {
  max_files: number;
  max_path_length: number;
}

export interface RenameRule {
  media_type: MediaType;
  template: string;
  is_default: boolean;
}
