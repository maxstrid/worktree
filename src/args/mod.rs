use std::{collections::HashMap, error::Error, hash::Hash, hash::Hasher};

pub trait MainArgument {
    fn name(&self) -> String;
    fn handle_sub_args(&self, sub_args: Vec<SubArgument>);
    fn register_sub_arg(&self, sub_arg: SubArgument);
    fn sub_args(&self) -> Vec<SubArgument>;
}

impl Hash for Box<dyn MainArgument> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.name().hash(state)
    }
}

impl PartialEq for Box<dyn MainArgument> {
    fn eq(&self, other: &Box<dyn MainArgument>) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Box<dyn MainArgument> {}

trait Argument {
    fn default() -> Self
    where
        Self: Sized;

    fn from_string(&self, value: &str) -> Box<dyn Argument>;
}

impl<T: From<String> + Default + 'static> Argument for T {
    fn default() -> Self {
        return T::default();
    }
    fn from_string(&self, value: &str) -> Box<dyn Argument> {
        return Box::new(T::from(value.to_string()));
    }
}

pub struct SubArgument {
    name: String,
    default_value: Option<Box<dyn Argument>>,
    description: Option<String>,
    short_name: Option<String>,
}

struct ArgumentParser<'a> {
    sub_arguments: HashMap<&'a Box<dyn MainArgument>, Vec<SubArgument>>,
}

impl<'a> ArgumentParser<'a> {
    pub fn new() -> Self {
        return ArgumentParser {
            sub_arguments: HashMap::new(),
        };
    }

    pub fn register_main_arg(&mut self, main_arg: &'a Box<dyn MainArgument>) {
        self.sub_arguments.insert(main_arg, main_arg.sub_args());
    }

    pub fn parse(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
