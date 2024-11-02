use std::collections::BTreeMap;

pub struct Pallet {
  balances: BTreeMap<String, u128>,
}

impl Pallet {
  pub fn new() -> Self {
    Self {
      balances: BTreeMap::new()
    }
  }

  pub fn set_balance(&mut self, account: &str, amount: u128) {
    self.balances.insert(account.to_string(), amount);
  }

  pub fn get_balance(&self, account: &str) -> u128 {
    *self.balances.get(account).unwrap_or(&0)
  }
}

#[test]
fn init_balances() {
  let mut balances = Pallet::new();

  assert_eq!(balances.get_balance(&"alice"), 0);
  balances.set_balance(&"alice", 100);
  assert_eq!(balances.get_balance(&"alice"), 100);
  assert_eq!(balances.get_balance(&"bob"), 0);
}