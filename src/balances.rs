use std::collections::BTreeMap;

pub struct Pallet {
  balances: BTreeMap<String, u128>,
}