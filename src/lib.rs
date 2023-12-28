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

//macro_rules! directory {
//    ( $( $key:expr => $value:expr ),* $(,)? ) => {
//        {
//            let mut map = std::collections::HashMap::new();
//            $(
//                map.insert($key.to_string(), $value);
//            )*
//            DirectoryContent::Directory(Directory(map))
//        }
//    };
//}
//
//macro_rules! fixture {
//    ( $( $key:expr => $value:expr ),* $(,)? ) => {
//        {
//            let mut map = std::collections::HashMap::new();
//            $(
//                map.insert($key.to_string(), $value);
//            )*
//            Directory(map)
//        }
//    };
//}

#[macro_export]
macro_rules! directory {
    // Recursive directory structure
    //( $( $key:expr => { $( $inner_key:expr => $inner_value:tt ),* $(,)? } ),* $(,)? ) => {
    //    {
    //        let mut map = std::collections::HashMap::new();
    //        $(
    //            map.insert($key.to_string(), DirectoryContent::Directory(Directory::new(directory! { $( $inner_key => $inner_value ),* })));
    //        )*
    //        DirectoryContent::Directory(Directory(map))
    //    }
    //};

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_macro_works() {
        //let macro_fixture = fixture! {
        //    "someDir" => directory! {
        //        "someFile" => DirectoryContent::File("contents of file here".to_string()),
        //    },
        //    "someOtherFile" => DirectoryContent::File("other contents".to_string())
        //};
        let macro_fixture = directory! {
            "someOtherFile" => "other contents",
            "otherFile" => "lol yesss",
        };

        //let mut sub_directory_contents: HashMap<String, DirectoryContent> = HashMap::new();
        //sub_directory_contents.insert(
        //    "someFile".to_string(),
        //    DirectoryContent::File("contents of file here".to_string()),
        //);

        let mut contents = HashMap::new();
        //contents.insert(
        //    "someDir".to_string(),
        //    DirectoryContent::Directory(Directory(sub_directory_contents)),
        //);
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
}
