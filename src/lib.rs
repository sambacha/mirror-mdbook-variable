#[macro_use]
extern crate lazy_static;
use mdbook::{
    book::{Book, BookItem},
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::{CaptureMatches, Captures, Regex};
use toml::value::{Table, Value};
#[derive(Default)]
pub struct VariablesPreprocessor;

impl VariablesPreprocessor {
    pub(crate) const NAME: &'static str = "variables";

    /// Create a new `LinkPreprocessor`.
    pub fn new() -> Self {
        VariablesPreprocessor
    }
}

impl Preprocessor for VariablesPreprocessor {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut variables = None;
        if let Some(config) = ctx.config.get_preprocessor(VariablesPreprocessor::NAME) {
            if let Some(vars) = config.get("variables") {
                variables = Some(vars);
            }
        }
        if let Some(Value::Table(vars)) = variables {
            book.for_each_mut(|section: &mut BookItem| {
                if let BookItem::Chapter(ref mut ch) = *section {
                    ch.content = replace_all(&ch.content, vars);
                }
            });
        }
        Ok(book)
    }
}

fn replace_all(s: &str, variables: &Table) -> String {
    // When replacing one thing in a string by something with a different length,
    // the indices after that will not correspond,
    // we therefore have to store the difference to correct this
    let mut previous_end_index = 0;
    let mut replaced = String::new();

    for variable in find_variables(s) {
        replaced.push_str(&s[previous_end_index..variable.start_index]);
        if let Some(value) = variables.get(&variable.name) {
            if let Value::String(s) = value {
                replaced.push_str(&s);
            } else {
                replaced.push_str(&value.to_string());
            }
        }
        previous_end_index = variable.end_index;
    }

    replaced.push_str(&s[previous_end_index..]);
    replaced
}

struct VariablesIter<'a>(CaptureMatches<'a, 'a>);

struct Variable {
    start_index: usize,
    end_index: usize,
    name: String,
}

impl Variable {
    fn from_capture(cap: Captures) -> Option<Variable> {
        let value = cap.get(1);
        value.map(|v| {
            cap.get(0)
                .map(|mat| Variable {
                    start_index: mat.start(),
                    end_index: mat.end(),
                    name: v.as_str().to_string(),
                })
                .expect("base match exists a this point ")
        })
    }
}

impl<'a> Iterator for VariablesIter<'a> {
    type Item = Variable;
    fn next(&mut self) -> Option<Variable> {
        for cap in &mut self.0 {
            return Variable::from_capture(cap);
        }
        None
    }
}

fn find_variables(contents: &str) -> VariablesIter {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").unwrap();
    }
    VariablesIter(RE.captures_iter(contents))
}

#[cfg(test)]
mod tests {
    use super::replace_all;
    use toml::value::{Table, Value};

    #[test]
    pub fn test_variable_replaced() {
        let to_replace = r" # Text {{var1}} \
            text \
            text {{var2}} \
            val  \
            (text {{var3}})[{{var3}}/other] \
        ";

        let mut table = Table::new();
        table.insert("var1".to_owned(), Value::String("first".to_owned()));
        table.insert("var2".to_owned(), Value::String("second".to_owned()));
        table.insert("var3".to_owned(), Value::String("third".to_owned()));

        let result = replace_all(to_replace, &table);

        assert_eq!(
            result,
            r" # Text first \
            text \
            text second \
            val  \
            (text third)[third/other] \
        "
        );
    }
}
