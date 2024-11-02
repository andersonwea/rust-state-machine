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

  pub fn transfer_balance(
    &mut self, 
    from: &str, 
    to: &str, 
    amount: u128
  ) -> Result<(), &'static str> {
    let from_balance = self.get_balance(&from);
    let to_balance = self.get_balance(&to);

    let new_from_balance = from_balance
      .checked_sub(amount)
      .ok_or("Insufficient balance.")?;

    let new_to_balance = to_balance
      .checked_add(amount)
      .ok_or("Overflow.")?;

    self.set_balance(&from, new_from_balance);
    self.set_balance(&to, new_to_balance);

    Ok(())
  }
}

mod test {
    use super::Pallet;

  #[test]
  fn init_balances() {
    let mut balances = Pallet::new();
  
    assert_eq!(balances.get_balance(&"alice"), 0);
    balances.set_balance(&"alice", 100);
    assert_eq!(balances.get_balance(&"alice"), 100);
    assert_eq!(balances.get_balance(&"bob"), 0);
  }
  
  #[test]
  fn transfer_balance() {
    let mut balances = Pallet::new();

    assert_eq!(balances.transfer_balance(&"alice", &"bob", 5), Err("Insufficient balance."));

    balances.set_balance(&"alice", 10);
    balances.set_balance(&"bob", 5);

    assert_eq!(balances.transfer_balance(&"alice", &"bob", 5), Ok(()));
    assert_eq!(balances.get_balance(&"alice"), 5);
    assert_eq!(balances.get_balance(&"bob"), 10);
  }

}