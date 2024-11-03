use std::collections::BTreeMap;

pub struct Pallet {
  block_number: u32,
  nonce: BTreeMap<String, u32>,
}

impl Pallet {
  pub fn new() -> Self {
    Self {
      block_number: 0,
      nonce: BTreeMap::new(),
    }
  }

  pub fn block_number(&self) -> u32 {
    self.block_number
  }

  pub fn inc_block_number(&mut self) {
    self.block_number += 1
  }

  pub fn inc_nonce(&mut self, account: &str) {
    let account_nonce = self.nonce.get(account).unwrap_or(&0);

    self.nonce.insert(account.to_string(), account_nonce + 1);
  }
}

mod test {
    use super::Pallet;

  #[test]
  fn init_system() {
    let mut system = Pallet::new();

    system.inc_block_number();
    system.inc_nonce(&"alice");

    assert_eq!(system.block_number(), 1);
    assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
  }
}