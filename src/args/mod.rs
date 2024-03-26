use std::{collections::HashMap, error::Error, rc::Rc};

struct MainArgument {
    name: &'static str,
    sub_arguments: Vec<Rc<SubArgument>>,
    sub_argument_handler: fn(Vec<SubArgument>) -> (),
}

impl MainArgument {
    pub fn new(name: &'static str, sub_argument_handler: fn(Vec<SubArgument>) -> ()) -> Self {
        return MainArgument {
            name,
            sub_argument_handler,
            sub_arguments: Vec::new(),
        };
    }

    pub fn handle_sub_arguments(&self, sub_args: Vec<SubArgument>) {
        (self.sub_argument_handler)(sub_args)
    }

    pub fn register_sub_argument(&mut self, argument: SubArgument) {
        self.sub_arguments.push(argument.into())
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn sub_arguments(&self) -> Vec<Rc<SubArgument>> {
        self.sub_arguments.clone()
    }
}

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

struct ArgumentParser {
    sub_arguments: HashMap<String, Vec<Rc<SubArgument>>>,
    main_arguments: HashMap<String, MainArgument>,
}

impl ArgumentParser {
    pub fn new() -> Self {
        return ArgumentParser {
            sub_arguments: HashMap::new(),
            main_arguments: HashMap::new(),
        };
    }

    pub fn register_main_arg(&mut self, main_arg: MainArgument) {
        self.sub_arguments
            .insert(String::from(main_arg.name()), main_arg.sub_arguments());
    }

    pub fn parse(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
