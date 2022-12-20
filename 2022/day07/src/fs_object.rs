#[derive(Debug, PartialEq)]
pub struct FileSystemObject {
    name: String,
    contents: Option<Vec<FileSystemObject>>,
    size: Option<usize>,
}

impl FileSystemObject {
    fn new_dir(name: &str) -> Self {
        FileSystemObject { name: name.to_owned(), contents: Some(vec![]), size: None }
    }

    fn new_file(name: &str, size: usize) -> Self {
        FileSystemObject { name: name.to_owned(), contents: None, size: Some(size) }
    }

    pub fn get_size(&self) -> usize {
        if let Some(val) = self.size {
            return val;
        }

        self.contents.as_ref().map_or(
            0,
            |contents| contents.iter().map(|fso| fso.get_size()).sum()
        )
    }

    /// Get the sum of the sizes of the folders smaller than 100000
    pub fn part_a(&self) -> usize {
        let mut sum = 0;
        if let Some(contents) = &self.contents {
            for fso in contents {
                if fso.size.is_none() {
                    let dir_size = fso.get_size();
                    if dir_size < 100000 {
                        sum += dir_size;
                    }
                    sum += fso.part_a();
                }
            }
        }
        sum
    }

    fn dir_sizes(&self) -> Vec<usize> {
        let mut dir_sizes = vec![];
        if let Some(contents) = &self.contents {
            for fso in contents {
                if fso.size.is_none() {
                    let dir_size = fso.get_size();
                    dir_sizes.push(dir_size);
                    dir_sizes.extend(fso.dir_sizes());
                }
            }
        }
        dir_sizes
    }

    pub fn part_b(&self) -> usize {
        let mut dir_sizes = self.dir_sizes();
        let size_to_free = self.get_size() - 40_000_000;
        dir_sizes.retain(|x| *x > size_to_free);
        dir_sizes.sort();
        *dir_sizes.first().unwrap()
    }
}

impl FromIterator<&'static str> for FileSystemObject {
    fn from_iter<I: IntoIterator<Item=&'static str>>(iter: I) -> Self {
        let mut node_stack: Vec<FileSystemObject> = vec![
            FileSystemObject::new_dir("/"),
        ];

        for line in iter {
            if line == "$ cd /" {
                continue;
            }
            if line == "$ ls" {
                continue;
            }

            if line.starts_with("$ cd ") {
                let dir_name = &line[5..];
                if dir_name == ".." {
                    if let Some(popped) = node_stack.pop() {
                        node_stack.last_mut().map(|last| last.contents.as_mut().unwrap().push(popped));
                    }
                    continue;
                }

                // Get the FSO with the same name from the last node and push it to stack
                let index = node_stack
                    .last()
                    .map(|fso| fso.contents.as_ref().unwrap().iter().position(|x| {
                        x.name == dir_name
                    }).unwrap())
                    .unwrap();
                let popped = node_stack.last_mut().map(|fso| fso.contents.as_mut().unwrap().remove(index)).unwrap();
                node_stack.push(popped);
                continue;
            }

            if line.starts_with("dir ") {
                let dir_name = &line[4..];
                let parsed = FileSystemObject::new_dir(dir_name);
                node_stack.last_mut().map(|last| last.contents.as_mut().unwrap().push(parsed));
                continue;
            }

            // Only remaining option is a file
            let size_raw = line.split_whitespace().next().unwrap();
            let size = size_raw.parse().unwrap();
            let name = line.split_whitespace().skip(1).next().unwrap();
            let parsed = FileSystemObject::new_file(name, size);
            node_stack.last_mut().map(|last| last.contents.as_mut().unwrap().push(parsed));
        }

        // Force the rest of the stack to pop
        while node_stack.len() > 1 {
            let popped = node_stack.pop().unwrap();
            node_stack.last_mut().map(|last| last.contents.as_mut().unwrap().push(popped));
        }

        node_stack.pop().unwrap()
    }
}
