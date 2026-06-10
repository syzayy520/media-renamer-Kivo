// src-tauri/src/tmdb_search/tests/mod.rs
// 测试模块
// 职责：定义 tmdb_search 测试子模块

#[cfg(test)]
mod validation_tests;

#[cfg(test)]
mod request_builder_tests;

#[cfg(test)]
mod response_dto_tests;

#[cfg(test)]
mod candidate_mapper_tests;

#[cfg(test)]
mod error_mapper_tests;

#[cfg(test)]
mod rate_limit_mapper_tests;

#[cfg(test)]
mod mock_transport_tests;

#[cfg(test)]
mod api_key_provider_tests;

#[cfg(test)]
mod command_contract_tests;
