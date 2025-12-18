pub use crate::backend::gatherer::{ProcessInfo, SysSpecifics, SysStats};
pub use crate::{FilterType, SortCriteria, SortType};
pub use eframe::egui::{self, Color32};
pub use egui_extras::{Column, TableBuilder};
pub use std::cmp::Ordering;
pub use std::collections::HashSet;
pub use std::path::Path;
pub use std::process::Command;

pub struct ViewConfig<'a> {
    pub criteria: SortCriteria,
    pub sort_type: SortType,
    pub filter: FilterType,
    pub username: &'a String,
    pub search: &'a String,
    pub open: &'a mut HashSet<u32>,
}

pub mod overview_drawer;
pub mod table_drawer;
pub mod tree_drawer;
