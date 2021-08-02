use std::hash::{Hash, Hasher};
use std::{collections::HashMap, fmt};

use super::Expression;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct HashLiteral {
    pub pairs: HashMap<Expression, Expression>,
}

// Had to implement Hash for this because HashMap doesn't. Doesn't matter what this is because
// a HashLiteral isn't a valid expression as a key in a monkey hash.
impl Hash for HashLiteral {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("hash not implemented for HashLiteral");
    }
}

impl fmt::Display for HashLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairs: Vec<String> = (&self.pairs)
            .into_iter()
            .map(|(k, v)| format!("{}:{}", k.to_string(), v.to_string()))
            .collect();
        write!(f, "{{{}}}", pairs.join(", "))
    }
}
