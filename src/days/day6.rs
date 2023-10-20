use anyhow::{Error, Result};
use std::{collections::HashMap, str::FromStr};

struct IdStore {
    lookup: HashMap<String, usize>,
}

impl IdStore {
    fn new() -> IdStore {
        IdStore {
            lookup: HashMap::new(),
        }
    }

    fn get(&self, id: &str) -> Option<usize> {
        return self.lookup.get(id).copied();
    }

    fn get_mut(&mut self, id: &str) -> usize {
        if let Some(&existing) = self.lookup.get(id) {
            return existing;
        } else {
            let previous_len = self.lookup.len();
            self.lookup.insert(id.to_owned(), previous_len);
            return previous_len;
        }
    }

    fn size(&self) -> usize {
        self.lookup.len()
    }
}

struct Edge {
    from: usize,
    to: usize,
}

impl Edge {
    fn parse(store: &mut IdStore, e: &str) -> Result<Edge> {
        let components: Vec<&str> = e.split(')').collect();
        let [from, to] = components.as_slice() else {
            return Err(Error::msg("Malformed edge"));
        };

        Ok(Edge {
            from: store.get_mut(from),
            to: store.get_mut(to),
        })
    }
}

struct Input {
    edges: Vec<Edge>,
    store: IdStore,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut store = IdStore::new();

        let maybe_edges: Result<Vec<Edge>> = s
            .trim()
            .split('\n')
            .map(|e| Edge::parse(&mut store, e))
            .collect();

        Ok(Input {
            edges: maybe_edges?,
            store,
        })
    }
}

mod parta {

    pub fn build_adj_list(input: &super::Input) -> Vec<Vec<usize>> {
        let mut result = vec![Vec::new(); input.store.size()];

        for e in &input.edges {
            result[e.from].push(e.to);
        }

        result
    }

    pub fn dfs(adj: &Vec<Vec<usize>>, cur: usize, depth: usize) -> usize {
        let mut ans = depth;

        for &next in &adj[cur] {
            ans += dfs(adj, next, depth + 1);
        }

        ans
    }
}

mod partb {

    pub fn build_adj_list(input: &super::Input) -> Vec<Vec<usize>> {
        let mut result = vec![Vec::new(); input.store.size()];

        for e in &input.edges {
            result[e.from].push(e.to);
            result[e.to].push(e.from);
        }

        result
    }

    pub fn dfs(
        adj: &Vec<Vec<usize>>,
        cur: usize,
        from: Option<usize>,
        goal: usize,
    ) -> Option<usize> {
        if cur == goal {
            return Some(0);
        }

        let mut ans = None;

        for &next in &adj[cur] {
            if from.is_some_and(|f| f == next) {
                continue;
            }

            ans = ans.or_else(|| dfs(adj, next, Some(cur), goal).map(|d| d + 1));
        }

        ans
    }
}

pub fn part_a(input: &str) -> Result<String> {
    let inp: Input = input.parse()?;
    let adj = parta::build_adj_list(&inp);
    let ans = parta::dfs(
        &adj,
        inp.store.get("COM").ok_or(Error::msg("Root not found"))?,
        0,
    );
    Ok(ans.to_string())
}

pub fn part_b(input: &str) -> Result<String> {
    let inp: Input = input.parse()?;
    let adj = partb::build_adj_list(&inp);
    let start = inp.store.get("YOU").ok_or(Error::msg("Start not found"))?;
    let end = inp.store.get("SAN").ok_or(Error::msg("Goal not found"))?;
    let dist = partb::dfs(&adj, start, None, end).ok_or(Error::msg("No path found"))?;
    let ans = dist - 2;
    Ok(ans.to_string())
}
