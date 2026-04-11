use rand::distr::Alphanumeric;
use rand::{Rng, rng};

use crate::features::subscriptions::domain::SubscriptionToken;

pub fn generate_subscription_token() -> SubscriptionToken {
    let mut rng = rng();
    let token = std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect();

    SubscriptionToken::new(token)
}
