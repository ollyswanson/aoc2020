use aoc07::{InvertedRulesDFS, Result, Rules, RulesSearch};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Part 1
    let rules: Rules = input.parse()?;
    let inverted_rules = rules.invert();
    let mut dfs = InvertedRulesDFS::new(&inverted_rules);
    dfs.traverse("shiny gold")?;
    // it can't contain itself so minus 1!
    let count = dfs.count_visited() - 1;
    println!("Part 1: {}", count);

    // Part 2
    let rules_search = RulesSearch::new(&rules);
    let count = rules_search.bags_needed("shiny gold");
    println!("Part 2: {}", count);

    Ok(())
}
