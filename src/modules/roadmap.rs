use super::activity::ActivityId;

#[derive(Debug)]
pub struct ActivityWithRelatedDependencies {
    pub activity_id: ActivityId,
    pub head_name: String,
    pub activity_id_head: Option<ActivityId>,
}

#[derive(Debug, PartialEq)]
pub struct ActivitySum {
    pub id: ActivityId,
    pub name: String,
}
#[derive(Debug, PartialEq)]
pub struct Phase {
    pub activities: Vec<ActivitySum>,
}

#[derive(Default)]
pub struct Roadmap {
    pub phases: Vec<Phase>,
}

impl Roadmap {
    pub fn new(acts_with_deps: Vec<ActivityWithRelatedDependencies>) -> Self {
        Self {
            phases: get_phases(acts_with_deps),
        }
    }
}

fn get_phases_recursive(
    acts_with_deps: Vec<ActivityWithRelatedDependencies>,
    previous_ids: Option<&Vec<ActivityId>>,
    head_vec: &mut Vec<Phase>,
) {
    if acts_with_deps.is_empty() {
        return;
    }
    let (heads, tails): (
        Vec<Option<ActivitySum>>,
        Vec<Option<ActivityWithRelatedDependencies>>,
    ) = acts_with_deps
        .into_iter()
        .map(|act| match act.activity_id_head {
            None => (
                Some(ActivitySum {
                    id: act.activity_id,
                    name: act.head_name,
                }),
                None,
            ),
            Some(head_id) => {
                if previous_ids.is_some() && previous_ids.unwrap().contains(&head_id) {
                    return (
                        Some(ActivitySum {
                            id: act.activity_id,
                            name: act.head_name,
                        }),
                        None,
                    );
                }
                (None, Some(act))
            }
        })
        .unzip();

    let heads: Vec<ActivitySum> = heads.into_iter().flatten().collect();
    let tails: Vec<ActivityWithRelatedDependencies> = tails.into_iter().flatten().collect();

    let head_ids: Vec<ActivityId> = heads.iter().map(|act| act.id).collect();

    head_vec.push(Phase { activities: heads });

    get_phases_recursive(tails, Some(&head_ids), head_vec)
}

fn get_phases(acts_with_deps: Vec<ActivityWithRelatedDependencies>) -> Vec<Phase> {
    let mut accumulator: Vec<Phase> = vec![];
    get_phases_recursive(acts_with_deps, None, &mut accumulator);

    accumulator
}

#[cfg(test)]
mod test {
    use crate::modules::roadmap::ActivityWithRelatedDependencies;

    use super::{get_phases, ActivitySum, Phase};

    #[test]
    fn test_get_phases_empty() {
        let acts_with_deps: Vec<ActivityWithRelatedDependencies> = vec![];
        let roadmap = get_phases(acts_with_deps);
        let expected: Vec<Phase> = vec![];
        assert_eq!(roadmap, expected)
    }

    #[test]
    fn test_get_phases_only_first_phase() {
        let acts_with_deps: Vec<ActivityWithRelatedDependencies> = vec![
            ActivityWithRelatedDependencies {
                head_name: "First A".to_string(),
                activity_id: 1,
                activity_id_head: None,
            },
            ActivityWithRelatedDependencies {
                head_name: "First B".to_string(),
                activity_id: 2,
                activity_id_head: None,
            },
        ];
        let roadmap = get_phases(acts_with_deps);
        let expected: Vec<Phase> = vec![Phase {
            activities: vec![
                ActivitySum {
                    name: "First A".to_string(),
                    id: 1,
                },
                ActivitySum {
                    name: "First B".to_string(),
                    id: 2,
                },
            ],
        }];
        assert_eq!(roadmap, expected)
    }

    #[test]
    fn test_get_phases_more_phase() {
        let acts_with_deps: Vec<ActivityWithRelatedDependencies> = vec![
            ActivityWithRelatedDependencies {
                head_name: "First A".to_string(),
                activity_id: 1,
                activity_id_head: None,
            },
            ActivityWithRelatedDependencies {
                head_name: "Second B".to_string(),
                activity_id: 2,
                activity_id_head: Some(1),
            },
            ActivityWithRelatedDependencies {
                head_name: "Third C".to_string(),
                activity_id: 3,
                activity_id_head: Some(2),
            },
            ActivityWithRelatedDependencies {
                head_name: "Second D".to_string(),
                activity_id: 4,
                activity_id_head: Some(1),
            },
        ];
        let roadmap = get_phases(acts_with_deps);
        let expected: Vec<Phase> = vec![
            Phase {
                activities: vec![ActivitySum {
                    name: "First A".to_string(),
                    id: 1,
                }],
            },
            Phase {
                activities: vec![
                    ActivitySum {
                        name: "Second B".to_string(),
                        id: 2,
                    },
                    ActivitySum {
                        name: "Second D".to_string(),
                        id: 4,
                    },
                ],
            },
            Phase {
                activities: vec![ActivitySum {
                    name: "Third C".to_string(),
                    id: 3,
                }],
            },
        ];
        assert_eq!(roadmap, expected)
    }
}
