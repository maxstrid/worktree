use std::fmt::Debug;

pub trait MainArgument<T>
where
    String: std::convert::From<T>,
    T: Clone + std::fmt::Debug,
{
    type ArgumentName;

    fn handle_sub_args(sub_args: Vec<SubArgument>);
    fn register_sub_arg(sub_arg: SubArgument);
}

trait Argument: Debug {
    fn default() -> Self
    where
        Self: Sized;

    fn from_string(&self, value: &str) -> Box<dyn Argument>;
}

impl<T: From<String> + Debug + Default + 'static> Argument for T {
    fn default() -> Self {
        return T::default();
    }
    fn from_string(&self, value: &str) -> Box<dyn Argument> {
        return Box::new(T::from(value.to_string()));
    }
}

#[derive(Debug)]
pub struct SubArgument {
    name: String,
    default_value: Option<Box<dyn Argument>>,
    description: Option<String>,
    short_name: Option<String>,
}

#[derive(Debug)]
struct ArgumentParser {
    sub_arguments: Vec<SubArgument>,
}
