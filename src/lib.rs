mod tmux;
use tmux::{launch_tmux};
use proc_macro::TokenStream;


fn _format(input: TokenStream) -> TokenStream {
    let mut var_vector: Vec<String> = Vec::new();
    let mut print_string = String::new();

    let mut buffer = String::new();
    let mut is_var: bool = false;
    let mut first: bool = true;
    for token in input.to_string().chars() {
        if first {
            if token != 'f' {
                let mut output = "std::format!(".to_owned();
                output.push_str(&input.to_string());
                output.push_str(")");
                let stream: TokenStream = output.parse().unwrap();
                return stream
            } else {
                first = false;
                continue
            }
        }
        let token: String = token.to_string();
        if token.eq("{") {
            is_var = true;
            print_string += "{:?}";
        } else if token.eq("}") {
            is_var = false;
            var_vector.push(buffer);
            buffer = String::new();
        } else if is_var {
            buffer.push_str(&token);
        } else {
            print_string.push_str(&token);
        }
    }

    let mut output = "std::format!(".to_owned();
    output.push_str(&print_string);
    output.push_str(", ");
    output.push_str(&var_vector.join(","));
    output.push_str(")");
    let stream: TokenStream = output.parse().unwrap();
    return stream
}


#[proc_macro]
pub fn format(input: TokenStream) -> TokenStream {
    _format(input)
}


#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    let formated: String = _format(input).to_string();
    let mut buffer = "std::print!(\"{}\", ".to_owned();
    buffer.push_str(&formated);
    buffer.push_str(")");
    let stream: TokenStream = buffer.parse().unwrap();
    return stream
}


#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    let formated: String = _format(input).to_string();
    let mut buffer = "std::println!(\"{}\", ".to_owned();
    buffer.push_str(&formated);
    buffer.push_str(")");
    let stream: TokenStream = buffer.parse().unwrap();
    return stream
}


#[cfg(test)]
mod tests {
}