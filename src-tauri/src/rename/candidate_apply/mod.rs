// rename/candidate_apply 模块 - TMDb 候选应用功能族
// 职责：将 TMDb 候选信息映射为重命名预览字段
// 委托：movie_candidate_apply / tv_candidate_apply / target_name_builder
// 禁止：直接 I/O、模板渲染、冲突检测 — 全部委托子模块

pub mod movie_candidate_apply;
pub mod preview_apply_result;
pub mod target_name_builder;
pub mod tv_candidate_apply;

pub use movie_candidate_apply::apply_movie_candidate;
pub use preview_apply_result::{PreviewApplyError, PreviewApplyResult};
pub use tv_candidate_apply::apply_tv_candidate;
