//! Strategies for UUID generation

use proptest::prelude::{any, Strategy};
use uuid::Uuid;

/// Generates random uuid
pub fn random_uuid() -> impl Strategy<Value = Uuid> {
    any::<u128>().prop_map(Uuid::from_u128)
}