use camino::Utf8PathBuf;
use miette::miette;
use winnow::ascii::space1;
use winnow::combinator::{delimited, opt, preceded, repeat, terminated};
use winnow::token::take_until;
use winnow::PResult;
use winnow::Parser;

use crate::ghci::loaded_module::LoadedModule;
use crate::ghci::ModuleSet;
use crate::normal_path::NormalPath;

use super::lines::until_newline;
use super::ShowPaths;

/// Parse `:show modules` output into a set of loaded modules with their paths.
///
/// The output format is:
/// ```
/// ModuleName ( /absolute/path/to/Module.hs, interpreted )
/// ```
pub fn parse_show_modules(search_paths: &ShowPaths, input: &str) -> miette::Result<ModuleSet> {
    let modules: Vec<_> = repeat(0.., show_module_line)
        .parse(input)
        .map_err(|err| miette!("{err}"))?;

    modules
        .into_iter()
        .map(|(name, path)| {
            let normal_path = NormalPath::new(path, &search_paths.cwd)?;
            Ok(LoadedModule::with_name(normal_path, name))
        })
        .collect()
}

fn show_module_line(input: &mut &str) -> PResult<(String, Utf8PathBuf)> {
    // Parse module name (everything before the first space)
    let module_name = take_until(1.., ' ').parse_next(input)?;
    let module_name = module_name.to_string();

    // Skip spaces
    let _ = space1.parse_next(input)?;

    // Parse the path within parentheses: "( /path/to/file.hs, interpreted )"
    let path = delimited(
        '(',
        preceded(opt(space1), terminated(take_until(1.., ','), ',')),
        // Skip everything after the comma until the closing paren
        (take_until(0.., ')'), ')'),
    )
    .parse_next(input)?;

    let path = Utf8PathBuf::from(path.trim());

    // Consume the rest of the line
    let _ = until_newline.parse_next(input)?;

    Ok((module_name, path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ghci::parse::ShowPaths;
    use crate::normal_path::NormalPath;
    use camino::Utf8PathBuf;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[test]
    fn test_parse_show_modules() {
        let show_paths = ShowPaths {
            cwd: Utf8PathBuf::from("/Users/test/project"),
            search_paths: vec![],
        };

        let input = indoc!(
            "
            Main             ( /Users/test/project/test-main/Main.hs, interpreted )
            MyLib            ( /Users/test/project/src/MyLib.hs, interpreted )
            MyModule         ( /Users/test/project/src/MyModule.hs, interpreted )
            TestMain         ( /Users/test/project/test/TestMain.hs, interpreted )
            "
        );

        let result = parse_show_modules(&show_paths, input).unwrap();
        let modules: HashSet<_> = result.into_iter().collect();

        assert_eq!(modules.len(), 4);

        // Check that we have the expected modules with correct paths
        let normal_path = |p: &str| NormalPath::new(p, &show_paths.cwd).unwrap();

        assert!(modules.contains(&LoadedModule::with_name(
            normal_path("/Users/test/project/test-main/Main.hs"),
            "Main".to_owned()
        )));
        assert!(modules.contains(&LoadedModule::with_name(
            normal_path("/Users/test/project/src/MyLib.hs"),
            "MyLib".to_owned()
        )));
        assert!(modules.contains(&LoadedModule::with_name(
            normal_path("/Users/test/project/src/MyModule.hs"),
            "MyModule".to_owned()
        )));
        assert!(modules.contains(&LoadedModule::with_name(
            normal_path("/Users/test/project/test/TestMain.hs"),
            "TestMain".to_owned()
        )));
    }

    #[test]
    fn test_parse_show_modules_with_object_code() {
        let show_paths = ShowPaths {
            cwd: Utf8PathBuf::from("/home/user/project"),
            search_paths: vec![],
        };

        let input = indoc!(
            "
            Lib              ( /home/user/project/src/Lib.hs, /home/user/project/dist/build/Lib.o )
            Main             ( /home/user/project/app/Main.hs, interpreted )
            "
        );

        let result = parse_show_modules(&show_paths, input).unwrap();
        let modules: HashSet<_> = result.into_iter().collect();

        assert_eq!(modules.len(), 2);

        let normal_path = |p: &str| NormalPath::new(p, &show_paths.cwd).unwrap();

        assert!(modules.contains(&LoadedModule::with_name(
            normal_path("/home/user/project/src/Lib.hs"),
            "Lib".to_owned()
        )));
        assert!(modules.contains(&LoadedModule::with_name(
            normal_path("/home/user/project/app/Main.hs"),
            "Main".to_owned()
        )));
    }
}
