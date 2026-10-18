use crate::parser;

pub fn func_to_li(func: &parser::FunctionDeclarationNode) -> String {
    // define function
    let mut out = ".method public hidebysig static ".to_string();

    // add return type
    out += type_to_string(&func.return_type).as_str();
    out += " ";
    // add funcfion identifier
    out += func.identifier.as_str();

    // add args
    out += "(";

    out += func
        .args
        .iter()
        .map(|arg| type_to_string(&arg.arg_type) + " " + arg.identifier.as_str())
        .collect::<Vec<String>>()
        .join(", ")
        .as_str();

    out += ")";

    out
}

fn type_to_string(t: &parser::Type) -> String {
    match t {
        parser::Type::Int => "int32",
        parser::Type::Void => "void",
    }
    .to_string()
}
