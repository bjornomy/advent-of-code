use std::collections::HashMap;
use std::io::Split;
use regex::Regex;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn from(s: &str) -> Self {
        let mut splits = s.split(" ");

        File {
            size: splits.next()
                .map(|s| s.parse::<usize>().expect("a number"))
                .expect("there is a file size"),
            name: String::from(splits.next().expect("there is a file name")),
        }
    }
}


enum LineStatement {
    CD(String),
    LS,
    File(File),
    Dir(String),
}

impl LineStatement {
    fn parse(s: &str) -> Self {
        let chars = s.chars().collect::<Vec<char>>();
        match chars[0] {
            '$' => {
                match chars[2] {
                    'c' => LineStatement::CD(String::from(&s[5..])),
                    'l' => LineStatement::LS,
                    _ => unreachable!()
                }
            }
            'd' => LineStatement::Dir(String::from(&s[4..])),
            // assume anything else is a file
            _ => LineStatement::File(File::from(s))
        }
    }
}

#[derive(Debug, Default)]
struct FileTree {
    pwd: String,
    files: Vec<File>,
    dirs: HashMap<String, FileTree>,
}

impl FileTree {
    fn add_file(&mut self, p: String, f: File) {
        let ap = match p.strip_prefix("/") {
            Some(np) => np,
            None => p.as_str()
        };

        if p != "/" && p.contains("/") {
            let parts: Vec<&str> = ap.splitn(2, "/").collect();
            let ft = self.dirs.get_mut(parts[0]).expect("");

            if parts.len() == 1 {
                ft.files.push(f);
            } else {
                let mut fol = "/".to_string();
                fol.push_str(parts[1]);

                ft.add_file(fol, f);
            }
        } else {
            self.files.push(f);
        }
    }

    fn add_dir(&mut self, p: String, pwd: Option<String>) {
        let ap = match p.strip_prefix("/") {
            Some(np) => np,
            None => p.as_str()
        };

        if ap.contains("/") {
            let parts: Vec<&str> = ap.splitn(2, "/").collect();
            let ft = self.dirs.get_mut(parts[0]).expect("");

            ft.add_dir(String::from(parts[1]), Some(p.clone()));
        } else {
            let mut ft = FileTree::default();
            if pwd.is_some() {
                ft.pwd = p.clone();
            }

            self.dirs.insert(String::from(ap), ft);
        }
    }

    fn size(&self) -> usize {
        let size_of_files: usize = self.files.iter()
            .map(|f| f.size)
            .sum();

        let size_of_folders: usize = self.dirs.values()
            .map(FileTree::size)
            .sum();

        size_of_files + size_of_folders
    }

    fn get_all_dirs(&self) -> Vec<&FileTree> {
        let mut dirs = Vec::from([self]);
        self.dirs.values().into_iter()
            .flat_map(|ft| ft.get_all_dirs())
            .for_each(|ft| dirs.push(ft));

        dirs
    }
}

fn parse_file_tree(input: &str) -> FileTree {
    let mut pwd: String = String::from("/");
    let last_path_matcher = Regex::new(r"/[a-z]*$").unwrap();

    let mut root: FileTree = FileTree {
        pwd: pwd.clone(),
        files: Vec::default(),
        dirs: HashMap::default(),
    };

    input.lines()
        .map(|l| LineStatement::parse(l))
        .for_each(|ls| {
            match ls {
                LineStatement::CD(path) => {
                    if path != "/" {
                        if path == ".." {
                            pwd = String::from(last_path_matcher.replace_all(pwd.as_str(), ""));
                            if pwd == "" {
                                pwd = String::from("/");
                            }
                        } else {
                            if pwd.len() > 1 {
                                pwd += "/";
                            }
                            pwd += &path;
                            root.add_dir(pwd.clone(), Some(path));
                        }
                    }
                }
                LineStatement::File(file) => {
                    root.add_file(pwd.clone(), file)
                }
                // We don't need LS or Dir
                _ => {}
            }
        });

    root
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let root = parse_file_tree(input);
        let sum: usize = root.get_all_dirs().into_iter()
            .filter(|ft| ft.size() <= 100000)
            .map(FileTree::size)
            .sum();

        println!("{:?}", sum);
    }
}

const FS_SIZE: usize = 70000000;
const FS_NEEDED: usize = 30000000;

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let root = parse_file_tree(input);

        let unused = FS_SIZE - root.size();
        let needed_to_free = FS_NEEDED - unused;

        let mut possible: Vec<&FileTree> = root.get_all_dirs().into_iter()
            .filter(|ft| ft.size() > needed_to_free)
            .collect();

        possible.sort_by(|l, r| l.size().cmp(&r.size()));
        let to_delete = possible[0];

        println!("{:?}", to_delete.size());
    }
}

const INPUT: &str = include_str!("input.txt");
//const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}
