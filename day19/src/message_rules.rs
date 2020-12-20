use std::collections::HashMap;

/// A rule for a message.
pub enum MessageRule {
    Char(char),
    OtherRule(usize),
    Or(usize, usize),
    And(usize, usize),
    And3(usize, usize, usize),
    AndOrAnd((usize, usize), (usize, usize)),
}

/// A set of rules for messages.
pub struct MessageRules {
    rules: HashMap<usize, MessageRule>,
}

impl MessageRules {
    /// Recursively checks if a rule is valid.
    ///
    /// # Arguments
    ///
    /// * `rule` - Rule to check against character.
    /// * `chars` - Characters to check with rule.
    /// * `i` - Position in the `chars` to start reading from.
    pub fn check_rule(&self, rule: &MessageRule, chars: &Vec<char>, i: usize) -> (bool, usize) {
        match rule {
            MessageRule::Char(ch) => {
                if i < chars.len() {
                    if chars[i] == *ch {
                        return (true, i + 1);
                    }
                }
                (false, 0)
            }
            MessageRule::OtherRule(rule_i) => self.check_rule(&self.rules[rule_i], chars, i),
            MessageRule::Or(r1, r2) => {
                let (success1, i1) = self.check_rule(&self.rules[r1], chars, i);
                if success1 {
                    return (true, i1);
                }

                let (success2, i2) = self.check_rule(&self.rules[r2], chars, i);
                if success2 {
                    return (true, i2);
                }

                (false, 0)
            }
            MessageRule::And(r1, r2) => {
                let (success1, i1) = self.check_rule(&self.rules[r1], chars, i);
                if success1 {
                    let (success2, i2) = self.check_rule(&self.rules[r2], chars, i1);
                    if success2 {
                        return (true, i2);
                    }
                }
                (false, 0)
            }
            MessageRule::And3(r1, r2, r3) => {
                let (success1, i1) = self.check_rule(&self.rules[r1], chars, i);
                if success1 {
                    let (success2, i2) = self.check_rule(&self.rules[r2], chars, i1);
                    if success2 {
                        let (success3, i3) = self.check_rule(&self.rules[r3], chars, i2);
                        if success3 {
                            return (true, i3);
                        }
                    }
                }
                (false, 0)
            }
            MessageRule::AndOrAnd((r1, r2), (r3, r4)) => {
                // Check first pair.
                let (success1, i1) = self.check_rule(&self.rules[r1], chars, i);
                if success1 {
                    let (success2, i2) = self.check_rule(&self.rules[r2], chars, i1);
                    if success2 {
                        return (true, i2);
                    }
                }

                // Check second pair.
                let (success3, i3) = self.check_rule(&self.rules[r3], chars, i);
                if success3 {
                    let (success4, i4) = self.check_rule(&self.rules[r4], chars, i3);
                    if success4 {
                        return (true, i4);
                    }
                }

                (false, 0)
            }
        }
    }

    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Adds a rule to the rule set.
    pub fn add_rule(&mut self, i: usize, rule: MessageRule) {
        self.rules.insert(i, rule);
    }

    /// Checks if a string matches the message rule set.
    pub fn check_string(&self, s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let (success, i) = self.check_rule(&self.rules[&0], &chars, 0);
        success && i == chars.len()
    }
}
