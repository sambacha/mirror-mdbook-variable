use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::errors::Error as MDBookError;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_variables::VariablesPreprocessor;
use std::io;
use std::process;

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let app = App::new("mdBook variables preprocessor")
        .version(VERSION)
        .author("Tglman")
        .about("A mdbook preprocessor which replaces {{variables}} with values configured in the book.tom")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        );

    if let Some(sub_args) = app.get_matches().subcommand_matches("supports") {
        handle_supports(sub_args);
    } else {
        let preprocessor = VariablesPreprocessor::new();
        if let Err(e) = handle_preprocessing(&preprocessor) {
            eprint!("{}", e);
            process::exit(1);
        }
    }
}
fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = renderer != "not-supported";

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), MDBookError> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}
