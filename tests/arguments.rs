#[cfg(test)]
mod tests {
    use clap::{error::ErrorKind, Parser};
    use x4::engine::args::*;

    #[test]
    fn test_no_input() {
        // Simulate running the CLI with no arguments.
        let args = Cli::try_parse_from::<&[&str], _>(&[]);
        assert!(args.is_err());
        assert_eq!(
            args.unwrap_err().kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
}
