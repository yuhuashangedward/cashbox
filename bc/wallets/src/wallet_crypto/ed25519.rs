use super::*;
use substrate_primitives::{ed25519, Pair, crypto::Ss58Codec};


pub struct Ed25519;

impl Crypto for Ed25519 {
    type Seed = [u8; 32];
    type Pair = ed25519::Pair;

    fn seed_from_phrase(phrase: &str, password: Option<&str>) -> Self::Seed {
        Sr25519::seed_from_phrase(phrase, password)
    }
    fn pair_from_seed(seed: &Self::Seed) -> Self::Pair { ed25519::Pair::from_seed(&seed.clone()) }

    fn pair_from_suri(suri: &str, password_override: Option<&str>) -> Self::Pair {
        ed25519::Pair::from_legacy_string(suri, password_override)
    }

    fn ss58_from_pair(pair: &Self::Pair) -> String { pair.public().to_ss58check() }
    fn public_from_pair(pair: &Self::Pair) -> Vec<u8> { (&pair.public().0[..]).to_owned() }
    fn seed_from_pair(pair: &Self::Pair) -> Option<&Self::Seed> { Some(pair.seed()) }
}