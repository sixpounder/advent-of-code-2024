use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct OrderingRule {
    before: i32,
    after: i32,
}

impl<S: AsRef<str>> From<S> for OrderingRule {
    fn from(value: S) -> Self {
        let chunks = value.as_ref().split("|").collect::<Vec<&str>>();
        Self {
            before: chunks
                .get(0)
                .expect("No chunk")
                .parse::<i32>()
                .expect("Not a number"),
            after: chunks
                .get(1)
                .expect("No chunk")
                .parse::<i32>()
                .expect("Not a number"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PagesForUpdate(Vec<i32>);

impl<S: AsRef<str>> From<S> for PagesForUpdate {
    fn from(value: S) -> Self {
        Self(
            value
                .as_ref()
                .split(",")
                .map(|n| n.parse::<i32>().expect("Not a number"))
                .collect(),
        )
    }
}

impl Deref for PagesForUpdate {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PagesForUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PagesForUpdate {
    pub fn median(&self) -> &i32 {
        self.get(self.len() / 2).expect("No median item")
    }
}

#[derive(Debug, Clone)]
pub struct SafetyManualUpdates {
    ordering_rules: Vec<OrderingRule>,
    pages_for_updates: Vec<PagesForUpdate>,
}

impl<S: AsRef<str>> From<S> for SafetyManualUpdates {
    fn from(value: S) -> Self {
        let content = value.as_ref();
        let mut ordering_rules: Vec<OrderingRule> = vec![];
        let mut pages_for_updates: Vec<PagesForUpdate> = vec![];

        let mut parse_phase = 0u8;
        for line in content.lines() {
            if line.is_empty() {
                // Signals that from here onward each line is a page list to
                // produce for each update
                parse_phase = 1;
                continue;
            }

            match parse_phase {
                0 => ordering_rules.push(OrderingRule::from(line)),
                1 => {
                    pages_for_updates.push(PagesForUpdate::from(line));
                }
                _ => (),
            }
        }

        SafetyManualUpdates {
            ordering_rules,
            pages_for_updates,
        }
    }
}

impl SafetyManualUpdates {
    pub fn correct_updates(&self) -> Vec<&PagesForUpdate> {
        self.pages_for_updates
            .iter()
            .map(|item| (item, self.check(item)))
            .filter(|result| result.1)
            .map(|result| result.0)
            .collect::<Vec<&PagesForUpdate>>()
    }

    pub fn corrected_updates(&self) -> Vec<PagesForUpdate> {
        self.pages_for_updates
            .iter()
            .map(|item| (item, self.check(item)))
            .filter(|result| !result.1)
            .map(|(incorred_update, _invalid)| {
                let mut reordered: PagesForUpdate = incorred_update.clone();
                let relevant_rules = self
                    .ordering_rules
                    .iter()
                    .filter(|rule| {
                        reordered.contains(&rule.before) || reordered.contains(&rule.after)
                    })
                    .cloned()
                    .collect::<Vec<OrderingRule>>();

                // The idea: for each number couple in the incorrect update seek if there is
                // a rule that specifies their ordering and, if so, swap them where needed. Repeat
                // for all couples.
                for idx in 0..reordered.len() {
                    let mut head = reordered[idx];
                    if idx != reordered.len() - 1 {
                        let mut repeat = true;
                        while repeat {
                            for j in (idx + 1)..reordered.len() {
                                let other = reordered[j];
                                if relevant_rules.contains(&OrderingRule {
                                    before: other,
                                    after: head,
                                }) {
                                    reordered.swap(idx, j);
                                    head = reordered[idx];
                                    repeat = true;
                                    break;
                                } else {
                                    repeat = false;
                                }
                            }
                        }
                    }
                }
                reordered
            })
            .collect::<Vec<PagesForUpdate>>()
    }

    pub fn check(&self, pages: &PagesForUpdate) -> bool {
        for (idx, num) in pages.iter().enumerate() {
            for other_num in &pages[idx..pages.len()] {
                if self
                    .ordering_rules
                    .iter()
                    .find(|item| item.before == *other_num && item.after == *num)
                    .is_some()
                {
                    return false;
                }
            }
        }

        true
    }
}
