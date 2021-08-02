use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{fmt, rc::Rc};

use crate::parser::node;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    String(String),
    Return(Rc<Return>),
    Function(Rc<Function>),
    Builtin(Builtin),
    Array(Rc<Array>),
    Hash(Rc<MonkeyHash>),
    Null,
    // CompiledFunction(Rc<CompiledFunction>),
    // Closure(Rc<Closure>),
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub store: HashMap<String, Rc<Object>>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }
    pub fn set(&mut self, name: String, val: Rc<Object>) {
        self.store.insert(name, val);
    }
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(obj) => Some(Rc::clone(obj)),
            // 值为空时向外层查找
            None => match &self.outer {
                Some(o) => o.borrow().get(name),
                _ => None,
            },
        }
    }
    pub fn new_enclosed(env: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(&env)),
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

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<node::IdentifierExpression>,
    pub body: node::BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

impl Function {
    fn inspect(&self) -> String {
        let params: Vec<String> = (&self.parameters)
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        format!(
            "fn({}) {{\n{}\n}}",
            params.join(", "),
            self.body.to_string()
        )
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Function) -> bool {
        // TODO: implement this, but it should never get used
        panic!("partial eq not implemented for function");
    }
}
impl Eq for Function {}
impl Hash for Function {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // we should never hash an array so should be fine
        panic!("hash for function not supported");
    }
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub enum Builtin {
    Len,
    Puts,
    First,
    Last,
    Rest,
    Push,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Object> {
        match name {
            "len" => Some(Object::Builtin(Builtin::Len)),
            "first" => Some(Object::Builtin(Builtin::First)),
            "last" => Some(Object::Builtin(Builtin::Last)),
            "rest" => Some(Object::Builtin(Builtin::Rest)),
            "push" => Some(Object::Builtin(Builtin::Push)),
            "puts" => Some(Object::Builtin(Builtin::Puts)),
            _ => None,
        }
    }

    pub fn apply(&self, args: &Vec<Rc<Object>>) -> Result<Rc<Object>, String> {
        match self {
            Builtin::Len => {
                if args.len() != 1 {
                    return Err("len takes only 1 array or string argument".to_string());
                }

                let arg = &*Rc::clone(args.first().unwrap());
                match arg {
                    Object::String(s) => Ok(Rc::new(Object::Int(s.len() as i64))),
                    Object::Array(a) => Ok(Rc::new(Object::Int(a.elements.len() as i64))),
                    obj => Err(format!(
                        "object {:?} not supported as an argument for len",
                        obj
                    )),
                }
            }
            Builtin::First => {
                if args.len() != 1 {
                    return Err("first takes only 1 array argument".to_string());
                }

                let arg = &*Rc::clone(args.first().unwrap());
                match arg {
                    Object::Array(a) => match a.elements.first() {
                        Some(el) => Ok(Rc::clone(el)),
                        None => Ok(Rc::new(Object::Null)),
                    },
                    obj => Err(format!(
                        "object {:?} not supported as an argument for first",
                        obj
                    )),
                }
            }
            Builtin::Last => {
                if args.len() != 1 {
                    return Err("last takes only 1 array argument".to_string());
                }

                let arg = &*Rc::clone(args.first().unwrap());
                match arg {
                    Object::Array(a) => match a.elements.last() {
                        Some(el) => Ok(Rc::clone(el)),
                        None => Ok(Rc::new(Object::Null)),
                    },
                    obj => Err(format!(
                        "object {:?} not supported as an argument for last",
                        obj
                    )),
                }
            }
            Builtin::Rest => {
                if args.len() != 1 {
                    return Err("rest takes only 1 array argument".to_string());
                }

                let arg = &*Rc::clone(args.first().unwrap());
                match arg {
                    Object::Array(a) => {
                        if a.elements.len() <= 1 {
                            Ok(Rc::new(Object::Array(Rc::new(Array { elements: vec![] }))))
                        } else {
                            let mut elements = a.elements.clone();
                            elements.remove(0);
                            Ok(Rc::new(Object::Array(Rc::new(Array { elements }))))
                        }
                    }
                    obj => Err(format!(
                        "object {:?} is not supported as an argument for rest",
                        obj
                    )),
                }
            }
            Builtin::Push => {
                if args.len() != 2 {
                    return Err("push takes an array and an object".to_string());
                }

                let array = &*Rc::clone(args.first().unwrap());
                let obj = Rc::clone(args.last().unwrap());

                // TODO: handle pushing objects like an array onto an array
                match array {
                    Object::Array(a) => {
                        let mut elements = a.elements.clone();
                        elements.push(obj);
                        Ok(Rc::new(Object::Array(Rc::new(Array { elements }))))
                    }
                    _ => Err("first argument to push must be an array".to_string()),
                }
            }
            Builtin::Puts => {
                for arg in args {
                    println!("{}", arg.inspect())
                }
                Ok(Rc::new(Object::Null))
            }
        }
    }

    pub fn string(&self) -> String {
        self.inspect()
    }

    fn inspect(&self) -> String {
        match self {
            Builtin::Len => "len".to_string(),
            Builtin::First => "first".to_string(),
            Builtin::Last => "last".to_string(),
            Builtin::Rest => "rest".to_string(),
            Builtin::Push => "push".to_string(),
            Builtin::Puts => "puts".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Array {
    pub elements: Vec<Rc<Object>>,
}

impl Array {
    fn inspect(&self) -> String {
        let elements: Vec<String> = (&self.elements)
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        format!("[{}]", elements.join(", "))
    }
}

impl Hash for Array {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // we should never hash an array so should be fine
        panic!("hash for array not supported");
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MonkeyHash {
    pub pairs: HashMap<Rc<Object>, Rc<Object>>,
}
impl MonkeyHash {
    fn inspect(&self) -> String {
        let pairs: Vec<String> = (&self.pairs)
            .into_iter()
            .map(|(key, value)| format!("{}: {}", key.inspect(), value.inspect()))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    }
}
impl Hash for MonkeyHash {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // should never happen
        panic!("hash not implmented for monkey hash");
    }
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::String(s) => s.clone(),
            Object::Return(r) => r.value.inspect(),
            Object::Function(f) => f.inspect(),
            Object::Builtin(b) => b.inspect(),
            Object::Array(a) => a.inspect(),
            Object::Hash(h) => h.inspect(),
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
