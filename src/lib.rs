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
    // Literal file entry
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), DirectoryContent::File($value.to_string()));
            )*
            DirectoryContent::Directory(Directory(map))
        }
    };
    // Recursive directory structure
    ( $( $key:expr => { $( $inner_key:expr => $inner_value:tt ),* $(,)? } ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), directory! { $( $inner_key => $inner_value ),* });
            )*
            DirectoryContent::Directory(Directory(map))
        }
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_macro_works_for_files() {
        let macro_fixture = directory! {
            "someOtherFile" => "other contents",
            "otherFile" => "lol yesss",
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

        let expected = DirectoryContent::Directory(Directory(contents));

        assert_eq!(macro_fixture, expected);
    }

    #[test]
    fn fixture_macro_works_for_empty_directories() {
        let macro_fixture = directory! {
            "someDir" => { },
        };

        let sub_directory_contents: HashMap<String, DirectoryContent> = HashMap::new();

        let mut contents = HashMap::new();
        contents.insert(
            "someDir".to_string(),
            DirectoryContent::Directory(Directory(sub_directory_contents)),
        );
        let expected = DirectoryContent::Directory(Directory(contents));

        assert_eq!(macro_fixture, expected);
    }

    #[test]
    fn fixture_macro_works_for_a_file_and_a_directory() {
        let macro_fixture = directory! {
            "someDir" => { },
            "someFile" => "file contents here",
        };

        let sub_directory_contents: HashMap<String, DirectoryContent> = HashMap::new();

        let mut contents = HashMap::new();
        contents.insert(
            "someFile".to_string(),
            DirectoryContent::File("file contents here".to_string()),
        );

        contents.insert(
            "someDir".to_string(),
            DirectoryContent::Directory(Directory(sub_directory_contents)),
        );
        let expected = DirectoryContent::Directory(Directory(contents));

        assert_eq!(macro_fixture, expected);
    }
}
