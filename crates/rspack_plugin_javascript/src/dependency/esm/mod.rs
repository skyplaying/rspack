mod harmony_compatibility_dependency;
pub use harmony_compatibility_dependency::*;
mod harmony_export_expression_dependency;
pub use harmony_export_expression_dependency::*;
mod harmony_export_imported_specifier_dependency;
pub use harmony_export_imported_specifier_dependency::*;
mod harmony_export_specifier_dependency;
pub use harmony_export_specifier_dependency::*;
mod harmony_import_dependency;
pub use harmony_import_dependency::*;
mod harmony_export_header_dependency;
mod harmony_import_specifier_dependency;
pub use harmony_export_header_dependency::*;
pub use harmony_import_specifier_dependency::*;
mod import_dependency;
pub use import_dependency::*;
use rspack_core::DependencyCategory;
pub fn create_resource_identifier_for_esm_dependency(request: &str) -> String {
  format!("{}|{}", DependencyCategory::Esm, &request)
}
