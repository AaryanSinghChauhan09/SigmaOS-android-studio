//! SigmaOS-android-studio: Android Studio IDE model.
//!
//! Real implementation: projects, modules, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub sdk_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub id: String,
    pub project_id: String,
    pub language: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AndroidStudioError {
    EmptyProjectName,
    DuplicateProject(String),
}

impl fmt::Display for AndroidStudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProjectName => write!(f, "project name must not be empty"),
            Self::DuplicateProject(id) => write!(f, "duplicate project {:?}", id),
        }
    }
}

impl std::error::Error for AndroidStudioError {}

pub struct AndroidStudio {
    pub name: String,
    projects: BTreeMap<String, Project>,
    modules: BTreeMap<String, Module>,
}

impl AndroidStudio {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), projects: BTreeMap::new(), modules: BTreeMap::new() }
    }

    pub fn add_project(&mut self, p: Project) -> Result<(), AndroidStudioError> {
        if p.name.trim().is_empty() { return Err(AndroidStudioError::EmptyProjectName); }
        let id = p.id.clone();
        if self.projects.contains_key(&id) {
            return Err(AndroidStudioError::DuplicateProject(id.clone()));
        }
        self.projects.insert(id, p);
        Ok(())
    }

    pub fn project_count(&self) -> usize { self.projects.len() }

    pub fn get_project(&self, id: &str) -> Option<&Project> { self.projects.get(id) }

    pub fn add_module(&mut self, m: Module) {
        self.modules.insert(m.id.clone(), m);
    }

    pub fn get_module(&self, id: &str) -> Option<&Module> { self.modules.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = AndroidStudio::new("AndroidStudio");
        assert_eq!(a.name, "AndroidStudio");
        assert_eq!(a.project_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = AndroidStudio::new("AndroidStudio");
        let project = Project { id: "p1".into(), name: "MyApp".into(), sdk_version: "34".into() };
        a.add_project(project).unwrap();
        assert_eq!(a.project_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = AndroidStudio::new("AndroidStudio");
        let project = Project { id: "x".into(), name: "".into(), sdk_version: "x".into() };
        assert!(a.add_project(project).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AndroidStudio::new("AndroidStudio");
        let project = Project { id: "x".into(), name: "X".into(), sdk_version: "x".into() };
        a.add_project(project).unwrap();
        let project2 = Project { id: "x".into(), name: "X".into(), sdk_version: "x".into() };
        assert!(a.add_project(project2).is_err());
    }
}
