use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Index;
use std::fs;
use std::env;
use std::path;


#[derive(Debug)]
struct DirectoryInfo {
    name: String,
    path: FileSystemPath,
    files: Vec<FileInfo>,
    directories: Vec<DirectoryInfo>
}

#[derive(Debug)]
struct FileInfo {
    name: String, 
    size: usize
}

#[derive(Debug)]
struct FileSystem {
    root: DirectoryInfo
}

type FileSystemPath = Vec<String>;

impl DirectoryInfo {
    fn recursive_size(&self) -> usize {
        let mut total_size = self.files.iter().map(|y| y.size).sum();
        for directory in self.directories.iter() {
            total_size += directory.recursive_size();
        }
        return total_size;
    }
}

impl FileSystem {
    fn new() -> Self {
        Self { 
            root: DirectoryInfo { name: "<root>".to_string(), path: FileSystemPath::new(),  files: Vec::new(), directories: Vec::new()}
        }
    }

    fn mut_directory_by_path(&mut self, path: &FileSystemPath) -> Option<&mut DirectoryInfo> {
        if path.is_empty() {
            return Some(&mut self.root); 
        }
        let mut current_directory = &mut self.root;
        for path_segment in path.iter() {

           let next_directory = current_directory.directories.iter_mut().find(|y| y.name == *path_segment);
           current_directory = next_directory?;
        }   
        return Some(current_directory);
    }

    fn directory_by_path(&self, path: &FileSystemPath) -> Option<&DirectoryInfo> {
        if path.is_empty() {
            return Some(&self.root); 
        }
        let mut current_directory = &self.root;
        for path_segment in path.iter() {

           let next_directory = current_directory.directories.iter().find(|y| y.name == *path_segment);
           current_directory = next_directory?;
        }   
        return Some(current_directory);
    }

    fn insert(&mut self, path: &FileSystemPath, files: &mut Vec<FileInfo>, directories: &mut Vec<DirectoryInfo>) {
        if let Some( directory) = self.mut_directory_by_path(path) {
            directory.directories.append(directories);
            directory.files.append(files);
        }
    }

    fn from_shell_output_file(path: &str) -> Result<FileSystem, &'static str> {
        let text = fs::read_to_string(path).expect("Error reading file");
        let command_blocks : Vec<&str> = text.split("\n$").map(|y| y.trim()).collect();
    
        let mut cwd = FileSystemPath::new();
        let mut fs = FileSystem::new();
        
        for command in command_blocks {
    
            if command.starts_with("cd ") {
                let destination = &command[3..];
                if destination == ".." {
                    cwd.pop();
                }else{
                    cwd.push(destination.to_string());
                }
            }
    
            if command.starts_with("ls") {
                let response_lines : Vec<&str>= command.split("\n").map(|y| y.trim()).skip(1).collect();
    
                let mut dirs : Vec<DirectoryInfo> = Vec::new();
                let mut files : Vec<FileInfo> = Vec::new();
                for response in response_lines {
                    if response.starts_with("dir ") {
                        let dir_name = &response[4..];
                        let mut dir_path = cwd.clone();
                        dir_path.push(dir_name.to_string());

                        let dir = DirectoryInfo { name: dir_name.to_string(), path: dir_path,  directories: Vec::new(), files: Vec::new() };
                        dirs.push(dir);
                    }else {
                        let file_result_parts :Vec<&str>= response.split(" ").collect();
                        if file_result_parts.len() != 2 {
                            continue;
                        }
                        let file_info = FileInfo {
                            name: file_result_parts[1].to_string(),
                            size: file_result_parts[0].parse::<usize>().expect("Error parsing size")
                        };
                        files.push(file_info);
                    }
                }
                fs.insert(&cwd, &mut files, &mut dirs);
            } 
        }
        return Ok(fs);
    }

    fn to_directory_size_map(&self) -> HashMap<FileSystemPath, usize> {
        let mut path_size_map : HashMap<FileSystemPath, usize> = HashMap::new();

        let mut directory_visit_stack : Vec<&DirectoryInfo> = Vec::new();
        directory_visit_stack.push(&self.root);

        while let Some(directory) = directory_visit_stack.pop() {
            path_size_map.insert(directory.path.clone(), directory.recursive_size());
            for dir in directory.directories.iter() {
                directory_visit_stack.push(dir);
            }
        }
        return path_size_map;
    }

}

fn riddle_part_one(file_path: &String)  {
   
    let fs = FileSystem::from_shell_output_file(&file_path).expect("Error parsing");
    let path_size_map = fs.to_directory_size_map();

    let sum_dirs_beneath_size : usize = path_size_map.iter()
                                        .filter(|entry| *entry.1 < 100000)
                                        .map(|entry| *entry.1).sum();

    println!("Sum of all dir size matching criteria: {}", sum_dirs_beneath_size);



    
}

fn riddle_part_two(file_path: &String) {

    let fs = FileSystem::from_shell_output_file(&file_path).expect("Error parsing");
    let mut path_size_map = fs.to_directory_size_map();


    let disk_space : usize = 70000000;
    let total_occupied_space = fs.root.recursive_size();
    let update_required_free_space : usize = 30000000;

 
    let directory_to_delete = path_size_map.iter()
    .filter(|(path, size)| disk_space - (total_occupied_space - *size) >= update_required_free_space)
    .map(|entry| *entry.1).min();

    println!("Deletion candidations: {:?}", directory_to_delete);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Provide the input text file!");
    }
    let riddle_num: u32 = args
        .get(1)
        .unwrap()
        .parse()
        .expect("Error parsing riddle num");

    match riddle_num {
        1 => {
            riddle_part_one(args.get(2).unwrap());
        }
        2 => {
            riddle_part_two(args.get(2).unwrap());
        }
        _ => {
            panic!("Unknown riddle part number");
        }
    };
}