use std::{collections::HashMap, hash::Hash};

pub struct Cacher<TFunc, TArg: Eq + Hash + Copy, TVal: Copy>
where TFunc: Fn(TArg) -> TVal
{
    calculation: TFunc,
    value: HashMap<TArg, TVal>,
}

impl<TFunc, TArg: Eq + Hash + Copy, TVal: Copy> Cacher<TFunc, TArg, TVal>
where TFunc: Fn(TArg) -> TVal
{
    pub fn new(calculation: TFunc) -> Cacher<TFunc, TArg, TVal> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: TArg) -> TVal {
        if let Some(val) = self.value.get(&arg) {
            *val
        } else {
            let new_val = (self.calculation)(arg);
            self.value.insert(arg, new_val);
            new_val
        }
    }
}