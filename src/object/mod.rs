use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    String(String),
    Return(Rc<Return>),
    // Function(Rc<Function>),
    // Builtin(Builtin),
    // Array(Rc<Array>),
    // Hash(Rc<MonkeyHash>),
    Null,
    // CompiledFunction(Rc<CompiledFunction>),
    // Closure(Rc<Closure>),
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub store: HashMap<String, Rc<Object>>,
    // pub outer: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }
    pub fn set(&mut self, name: String, val: Rc<Object>) {
        self.store.insert(name, val);
    }
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(obj) => Some(Rc::clone(obj)),
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Return {
    pub value: Rc<Object>,
}
impl PartialEq for Return {
    fn eq(&self, _other: &Return) -> bool {
        unimplemented!("partial eq not implemented for Return")
    }
}

impl Eq for Return {}
impl Hash for Return {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // we should never hash an array so should be fine
        panic!("hash for return not supported");
    }
}
impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Return(r) => r.value.inspect(),
            // Object::Function(f) => f.inspect(),
            // Object::Builtin(b) => b.inspect(),
            // Object::Array(a) => a.inspect(),
            // Object::Hash(h) => h.inspect(),
            Object::Null => String::from("null"),
            // Object::CompiledFunction(f) => f.inspect(),
            // Object::Closure(c) => c.inspect(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inspect())
    }
}
