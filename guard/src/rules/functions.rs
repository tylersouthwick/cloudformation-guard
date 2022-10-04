pub(crate) mod strings;
pub(crate) mod collections;

use crate::rules::{QueryResult, Result};
use crate::rules::errors::{Error, ErrorKind};
use crate::rules::path_value::PathAwareValue;
use crate::rules::path_value::Path;

use std::convert::TryInto;

fn do_string_to_int(s : &String) -> Result<i64> {
    Ok(s.parse::<i64>()?)
}
fn string_to_int(parameters : &Vec<&PathAwareValue>) -> Result<PathAwareValue> {
    if parameters.len() != 1 {
        return Err(Error::new(ErrorKind::IncompatibleError(format!("unable to resolve function {}. Expected {} parameter, got {}", "string_to_int", 1, parameters.len()))))
    }
    use PathAwareValue::*;
    match &parameters[0] {
        &String((_, s)) => {
            println!("invoke as {}", s);
            let result : PathAwareValue = do_string_to_int(s)?.try_into()?;
            return Ok(result)
        }
        _ => {}
    };
    Err(Error::new(ErrorKind::IncompatibleError(format!("unable to resolve function {}. Expected {} parameter, got {}", "String", 1, parameters[0]))))
}

pub(crate) fn resolve_function(name : String, parameters : &Vec<&PathAwareValue>) -> Result<PathAwareValue> { //Result<QueryResult<'value>> {
    let result = match name.as_str() {
        "string_to_int" => string_to_int(parameters),
        _ => Err(Error::new(ErrorKind::IncompatibleError(format!("unable to resolve function {}", name))))
    }?.clone();
    //return Ok(QueryResult::Literal(&result));
    //let literal = PathAwareValue::String((Path::root(), "*".to_string()));
    Ok(result)
        //Err(Error::new(ErrorKind::IncompatibleError(format!("unable to resolve function {} - result {:?}", name, result))))
    //return Ok(QueryResult::Resolved(literal));
}
