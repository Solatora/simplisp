use crate::expression::Expression;
#[derive(Clone)]
pub struct Env {
    pub local: std::collections::HashMap<String, Expression>,
}
impl Env {
    pub fn std_lib() -> Env {
        let mut env = Env {
            local: std::collections::HashMap::new(),
        };
        env.local
            .insert("t".to_string(), Expression::Symbol("t".to_string()));
        env.local
            .insert("true".to_string(), Expression::Symbol("t".to_string()));
        env.local.insert("false".to_string(), Expression::Nil);
        env
    }
}
