//! Generate raw Rustdoc comments from a given [`crate::ast::ParsedDoxygen`]
//!
//! **The functions and structs here should _not_ be considered stable**

use crate::ast::ParsedDoxygen;

/// Generate raw Rustdoc comments from a given [`crate::ast::ParsedDoxygen`]
///
/// # Examples
/// ```
/// use doxygen_rs::ast::generate_ast;
/// use doxygen_rs::generator::generate_rustdoc;
/// use doxygen_rs::parser::parse_comment;
///
/// let parsed = parse_comment("@brief Random comment");
/// let ast = generate_ast(parsed);
/// let rustdoc = generate_rustdoc(ast);
/// ```
pub fn generate_rustdoc(doxygen: ParsedDoxygen) -> String {
    let mut rustdoc = String::new();

    if let Some(title) = doxygen.title {
        rustdoc += format!("# {}\n\n", title).as_str();
    }

    if let Some(deprecated) = doxygen.deprecated {
        if let Some(message) = deprecated.message {
            rustdoc += format!("**Warning!** This is deprecated! - {}", message).as_str();
        } else {
            rustdoc += "**Warning!** This is deprecated!".to_string().as_str();
        }
        rustdoc += "\n\n";
    }

    if let Some(brief) = doxygen.brief {
        rustdoc += &brief.to_string();
        rustdoc += "\n\n";
    }

    if let Some(description) = doxygen.description {
        for desc in description {
            rustdoc += format!("{}\n", desc).replace("< ", "").as_str();
        }
        rustdoc += "\n";
    }

    if let Some(warnings) = doxygen.warnings {
        rustdoc += "**Warning!**\n\n";
        for warning in warnings {
            rustdoc += format!("{:1}\n", warning.0).as_str();
        }
        rustdoc += "\n"
    }

    if let Some(returns) = doxygen.returns {
        rustdoc += "Returns:\n\n";
        for return_val in returns {
            rustdoc += format!("{:1}\n", return_val.0).as_str()
        }
        rustdoc += "\n";
    }

    if let Some(params) = doxygen.params {
        rustdoc += "# Arguments\n\n";
        for param in params {
            let mut dir = String::new();
            if let Some(direction) = param.direction {
                dir += format!(" [Direction: {}] ", direction.clone()).as_str()
            } else {
                dir += " "
            };

            if let Some(description) = param.description {
                rustdoc += format!("* `{}` -{}{:#1}", param.arg_name, dir, description).as_str();
            } else {
                rustdoc += format!("* `{}` -{}", param.arg_name, dir).as_str();
            }

            rustdoc += "\n";
        }

        rustdoc += "\n";
    }

    if let Some(return_values) = doxygen.return_values {
        rustdoc += "# Return values\n";
        for return_val in return_values {
            rustdoc += format!("{:1}\n", return_val.0).as_str();
        }
        rustdoc += "\n"
    }

    if let Some(notes) = doxygen.notes {
        rustdoc += "# Notes\n\n";
        for note in notes {
            rustdoc += format!("{:1}\n", note.0).as_str();
        }
        rustdoc += "\n"
    }

    if let Some(todos) = doxygen.todos {
        rustdoc += "# To Do\n\n";
        for todo in todos {
            rustdoc += format!("{:1}", todo).as_str();
        }

        rustdoc += "\n";
    }

    rustdoc
}
