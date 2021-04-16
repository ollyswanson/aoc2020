use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Graph of contains relationships, e.g. "muted coral bags contain 1 bright magenta bag, 1 dim aqua bag"
#[derive(Clone, Debug)]
pub struct Rules {
    inner: HashMap<String, Vec<(u32, String)>>,
}

impl Rules {
    pub fn invert(&self) -> InvertedRules<'_> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

        for (container, contained) in self.inner.iter() {
            for (_, name) in contained {
                let entry = map.entry(name).or_insert_with(|| Vec::new());
                entry.push(container);
            }
        }

        InvertedRules::new(map)
    }
}

impl FromStr for Rules {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        static CONTAINER: OnceCell<Regex> = OnceCell::new();
        static CONTAINED: OnceCell<Regex> = OnceCell::new();

        // muted coral bags contain 1 bright magenta bag, 1 dim aqua bag.
        let container_exp =
            CONTAINER.get_or_init(|| Regex::new(r"^(?P<name>[a-z]+\s[a-z]+) bags").unwrap());
        let contained_exp = CONTAINED.get_or_init(|| {
            Regex::new(r"\b(?P<qty>[0-9]+)\s(?P<name>[a-z]+\s[a-z]+)\s(:?bags|bag)").unwrap()
        });

        let mut graph = HashMap::new();

        for line in s.lines() {
            let container = container_exp.captures(line).ok_or("No container")?["name"].to_string();
            let contained = contained_exp
                .captures_iter(line)
                .map(|cap| match cap["qty"].parse() {
                    Ok(qty) => Ok((qty, cap["name"].to_string())),
                    Err(_) => Err("Invalid expression".into()),
                })
                .collect::<Result<Vec<_>>>()?;

            graph.insert(container, contained);
        }

        Ok(Self { inner: graph })
    }
}

pub struct RulesSearch<'a> {
    rules: &'a Rules,
}

impl<'a> RulesSearch<'a> {
    pub fn new(rules: &'a Rules) -> Self {
        Self { rules }
    }

    // counts the bags needed + 1 (it counts itself)
    fn count_bags(&self, cur: &str) -> u32 {
        let mut count = 1;

        if let Some(adjacent) = self.rules.inner.get(cur) {
            for (qty, name) in adjacent.iter() {
                count += self.count_bags(name) * qty;
            }
        }

        count
    }

    pub fn bags_needed(&self, target: &str) -> u32 {
        self.count_bags(target) - 1
    }
}

/// Graph of the inverted relationships. "aqua bag is contained by muted coral bags"
#[derive(Clone, Debug)]
pub struct InvertedRules<'a> {
    pub inner: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> InvertedRules<'a> {
    fn new(map: HashMap<&'a str, Vec<&'a str>>) -> Self {
        Self { inner: map }
    }
}

pub struct InvertedRulesDFS<'a> {
    graph: &'a InvertedRules<'a>,
    visited: HashMap<&'a str, bool>,
}

impl<'a> InvertedRulesDFS<'a> {
    pub fn new(inverted_rules: &'a InvertedRules) -> Self {
        let mut visited = HashMap::new();

        for (&k, v) in inverted_rules.inner.iter() {
            visited.insert(k, false);
            for &s in v {
                visited.insert(s, false);
            }
        }

        Self {
            graph: inverted_rules,
            visited,
        }
    }

    pub fn traverse(&mut self, current: &str) -> Result<()> {
        let visit = self
            .visited
            .get_mut(current)
            .ok_or_else(|| format!("{} not in graph", current))?;

        *visit = true;

        if let Some(adjacent) = self.graph.inner.get(current) {
            for &next in adjacent {
                if let Some(visited) = self.visited.get(next) {
                    if !visited {
                        self.traverse(next)?;
                    }
                };
            }
        }

        Ok(())
    }

    pub fn count_visited(&self) -> u32 {
        self.visited.values().fold(0, |acc, &visited| {
            if visited {
                return acc + 1;
            }
            acc
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn graph_from_str() {
        let s = "muted coral bags contain 1 bright magenta bag, 1 dim aqua bag.";
        let mut map = HashMap::new();
        map.insert(
            String::from("muted coral"),
            vec![
                (1, String::from("bright magenta")),
                (1, String::from("dim aqua")),
            ],
        );

        let graph: Rules = s.parse().unwrap();

        assert_eq!(map, graph.inner);
    }

    #[test]
    fn part_1() {
        let s = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let rules: Rules = s.parse().unwrap();
        let inverted_rules = rules.invert();
        let mut dfs = InvertedRulesDFS::new(&inverted_rules);
        dfs.traverse("shiny gold").unwrap();
        assert_eq!(dfs.count_visited() - 1, 4);
    }

    #[test]
    fn part_2() {
        let s = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let rules = s.parse().unwrap();
        let rules_search = RulesSearch::new(&rules);
        assert_eq!(rules_search.bags_needed("shiny gold"), 32);
    }

    #[test]
    fn part_2_alt() {
        let s = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

        let rules = s.parse().unwrap();
        let rules_search = RulesSearch::new(&rules);
        assert_eq!(rules_search.bags_needed("shiny gold"), 126);
    }
}
