use std::collections::HashMap;

#[derive(Debug)]
pub struct Directory(HashMap<String, DirectoryContent>);

#[derive(Debug)]
pub enum DirectoryContent {
    File(String),
    Directory(Directory),
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for DirectoryContent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DirectoryContent::File(a), DirectoryContent::File(b)) => a == b,
            (DirectoryContent::Directory(a), DirectoryContent::Directory(b)) => a == b,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! directory {
    // argument order:
    // 1. map
    // 3. remaining tokens

    // @directory
    // empty directory entry
    //(@directory $map:ident ()) => {};

    // // Next value is a directory.
    // (@directory $map:ident ($($key:expr)+) (=> {$($dir:tt)*} $($rest:tt)*)) => {
    //     let next_value = directory!({ $($dir)* });
    //     let _ = $map.insert(($($key)+).into(), next_value);
    //     directory!(@directory $map [$($key)+] (directory!({$($dir)*})) $($rest)*);
    // };

    // Next value is a string, last item
    (@directory $map:ident $($key:expr => $value:expr)+ $(,)?) => {
        let _ = $map.insert(($($key)+).into(), DirectoryContent::File(($($value)+).into()));

        DirectoryContent::Directory(Directory(map))
    };

    // Next value is a string followed by a comma
    (@directory $map:ident $($key:expr => $value:expr, $rest:tt)*) => {
        let _ = $map.insert(($key).into(), DirectoryContent::File($value.into()));

        directory!(@directory $map $($rest)*);
    };

    // // Recursive directory structure
    // ( $( $key:expr => { $( $inner_key:expr => $inner_value:tt ),* $(,)? } ),* $(,)? ) => {
    //     {
    //         let mut map = std::collections::HashMap::new();
    //         $(
    //             map.insert($key.to_string(), directory! { $( $inner_key => $inner_value ),* });
    //         )*
    //         DirectoryContent::Directory(Directory(map))
    //     }
    // };
    //
    // Literal file entry
    // (@directory $map:ident $( $key:expr => $value:expr, $($rest:tt)*)+) => {
    //     {
    //         $(
    //             map.insert($key.into(), DirectoryContent::File($value.to_string()));
    //         )*
    //         DirectoryContent::Directory(Directory(map))
    //     }
    // };

}

#[macro_export]
macro_rules! fixture {
    ($($tt:tt)+) => {{
        let mut map = std::collections::HashMap::new();

        directory!(@directory map $($tt)+)
    }};

    // handle empty invocation:
    // directory! {}
     () => {{
         let mut map: HashMap<String, DirectoryContent> = std::collections::HashMap::new();
         Directory(map)
     }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_macro_works_for_files() {
        let macro_fixture = fixture! {
            "someOtherFile" => "other contents",
            //"otherFile" => "lol yesss",
        };

        let mut contents = HashMap::new();
        contents.insert(
            "someOtherFile".to_string(),
            DirectoryContent::File("other contents".to_string()),
        );
        contents.insert(
            "otherFile".to_string(),
            DirectoryContent::File("lol yesss".to_string()),
        );

        let expected = Directory(contents);

        assert_eq!(macro_fixture, expected);
    }

    #[test]
    fn fixture_macro_without_contents_works() {
        let macro_fixture = fixture! {};

        let contents = HashMap::new();

        assert_eq!(macro_fixture, Directory(contents));
    }

    // #[test]
    // fn fixture_macro_works_for_empty_directories() {
    //     let macro_fixture = directory! {
    //         "someDir" => { },
    //     };
    //
    //     let sub_directory_contents: HashMap<String, DirectoryContent> = HashMap::new();
    //
    //     let mut contents = HashMap::new();
    //     contents.insert(
    //         "someDir".to_string(),
    //         DirectoryContent::Directory(Directory(sub_directory_contents)),
    //     );
    //     let expected = DirectoryContent::Directory(Directory(contents));
    //
    //     assert_eq!(macro_fixture, expected);
    // }

    // #[test]
    // fn fixture_macro_works_for_a_file_and_a_directory() {
    //     let macro_fixture = directory! {
    //         "someDir" => { },
    //         "someFile" => "file contents here",
    //     };
    //
    //     let sub_directory_contents: HashMap<String, DirectoryContent> = HashMap::new();
    //
    //     let mut contents = HashMap::new();
    //     contents.insert(
    //         "someFile".to_string(),
    //         DirectoryContent::File("file contents here".to_string()),
    //     );
    //
    //     contents.insert(
    //         "someDir".to_string(),
    //         DirectoryContent::Directory(Directory(sub_directory_contents)),
    //     );
    //     let expected = DirectoryContent::Directory(Directory(contents));
    //
    //     assert_eq!(macro_fixture, expected);
    // }
}
