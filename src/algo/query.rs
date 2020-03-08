pub struct Pair(pub usize, pub usize);

pub struct Multiqueries(Vec<Vec<usize>>);

impl Multiqueries {
  pub fn new() -> Multiqueries {
    Multiqueries(Vec::new())
  }

  pub fn insert(&mut self, query: Vec<usize>) {
    self.0.push(query);
  }

  pub fn pairs(&self) -> Vec<Pair> {
    let mut dimensions = Vec::new();
    for query in &self.0 {
      for val in query {
        dimensions.push(*val);
      }
    }
    dimensions.sort();
    dimensions.dedup();

    let mut pairs = Vec::new();

    for di in &dimensions {
      for dj in &dimensions {
        if dj <= di { continue; }
        pairs.push(Pair(*di, *dj));
      }
    }

    pairs
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn multiqueries() {
    let mut m = Multiqueries::new();
    m.insert(vec![1, 2, 3]);
    m.insert(vec![2, 3]);
    m.insert(vec![2, 3, 6]);

    let pairs = m.pairs();
    assert_eq!(pairs.len(), 6);

    let p1 = pairs.get(0).unwrap();
    assert_eq!(p1.0, 1);
    assert_eq!(p1.1, 2);
  }
}
