use std::collections::HashMap;

use crate::pipfile_lock::Dependency;

#[derive(Clone, Debug, Default)]
pub struct Diff {
    pub changed: HashMap<String, (Dependency, Dependency)>,
    pub new: HashMap<String, Dependency>,
    pub deleted: HashMap<String, Dependency>,
}

impl Diff {
    pub fn compare_dependencies(
        new: &HashMap<String, Dependency>,
        old: &HashMap<String, Dependency>,
    ) -> Self {
        let mut diff = Diff::default();

        for (dep_name, new_dep) in new.iter() {
            if let Some(old_dep) = old.get(dep_name) {
                if new_dep != old_dep {
                    diff.changed
                        .insert(dep_name.clone(), (new_dep.clone(), old_dep.clone()));
                }
            } else {
                diff.new.insert(dep_name.clone(), new_dep.clone());
            }
        }

        for (dep_name, old_dep) in old.iter() {
            if new.get(dep_name).is_none() {
                diff.deleted.insert(dep_name.clone(), old_dep.clone());
            }
        }

        diff
    }
}

pub fn print_diff(diff: &Diff) {
    if !diff.changed.is_empty() {
        println!("Changed:");
        for (dep_name, (new, old)) in diff.changed.iter() {
            println!("  {dep_name}: {} => {}", old, new);
        }
    }

    if !diff.new.is_empty() {
        println!("New:");
        for (dep_name, new) in diff.new.iter() {
            println!("  {dep_name}: {}", new);
        }
    }

    if !diff.deleted.is_empty() {
        println!("Deleted:");
        for (dep_name, deleted) in diff.deleted.iter() {
            println!("  {dep_name}: {}", deleted);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_dependencies() {
        let new = HashMap::from([
            (
                "lib1".to_owned(),
                Dependency::Pip {
                    version: "3.2.0".to_owned(),
                },
            ),
            (
                "lib2".to_owned(),
                Dependency::Git {
                    git_ref: "abcdef".to_owned(),
                },
            ),
            (
                "lib3".to_owned(),
                Dependency::Pip {
                    version: "1.2".to_owned(),
                },
            ),
        ]);
        let old = HashMap::from([
            (
                "lib1".to_owned(),
                Dependency::Pip {
                    version: "3.2.1".to_owned(),
                },
            ),
            (
                "lib2".to_owned(),
                Dependency::Git {
                    git_ref: "abcdef".to_owned(),
                },
            ),
            (
                "lib4".to_owned(),
                Dependency::Pip {
                    version: "0.1".to_owned(),
                },
            ),
        ]);

        let diff = Diff::compare_dependencies(&new, &old);

        assert_eq!(
            diff.changed,
            HashMap::from([(
                "lib1".to_owned(),
                (
                    Dependency::Pip {
                        version: "3.2.0".to_owned(),
                    },
                    Dependency::Pip {
                        version: "3.2.1".to_owned(),
                    },
                )
            )])
        );

        assert_eq!(
            diff.new,
            HashMap::from([(
                "lib3".to_owned(),
                Dependency::Pip {
                    version: "1.2".to_owned(),
                },
            )])
        );

        assert_eq!(
            diff.deleted,
            HashMap::from([(
                "lib4".to_owned(),
                Dependency::Pip {
                    version: "0.1".to_owned(),
                },
            )])
        );
    }
}
