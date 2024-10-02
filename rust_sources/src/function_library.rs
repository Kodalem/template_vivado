use std::fs;
use std::path::Path;
use std::time::SystemTime;
#[derive(Clone)]
struct ProjectFile {
    file_name: String,
    file_path: String,
    file_age: SystemTime,
    exists: bool,
}

impl ProjectFile {
    fn new(file_name: String, file_path: String, file_age: SystemTime, exists: bool) -> ProjectFile {
        ProjectFile {
            file_name,
            file_path,
            file_age,
            exists,
        }
    }
    fn update_file_age(&mut self, new_file_age: SystemTime) {
        self.file_age = new_file_age;
    }
}

#[derive(Clone)]
struct SymbolicLinkStructFile{
    repo_file: ProjectFile,
    project_file: ProjectFile,
    is_linked: bool,
    source_type: String,
}
#[derive(Clone)]
struct FileStructure {
    design: Vec<SymbolicLinkStructFile>,
    constraint: Vec<SymbolicLinkStructFile>,
    simulation: Vec<SymbolicLinkStructFile>,
}


fn overwrite_file(from_file: &mut ProjectFile, to_file: &mut ProjectFile) -> bool {
    // Save the old file age 
    let old_file_age = from_file.file_age;
    
    
    let from_file_path = Path::new(&from_file.file_path);
    //println!("From file: {}", from_file.file_path);
    let to_file_path = Path::new(&to_file.file_path);
    //println!("To file:{}", to_file.file_path);
    fs::copy(from_file_path, to_file_path).expect("Something went wrong while copying the file");
    
    // Update the file age
    let metadata = fs::metadata(to_file.file_path.clone()).unwrap();
    let modified_time = metadata.modified().unwrap();
    
    // Throw error if the file age has not been updated
    if modified_time == old_file_age {
        panic!("The file age has not been updated");
    }
    
    to_file.update_file_age(modified_time);
    from_file.update_file_age(modified_time);
    
    true
}

fn check_if_sources_exist() -> bool {
    let sim_path = Path::new("../simulation_sources");
    let des_path = Path::new("../design_sources");
    let cont_path = Path::new("../constraint_sources");

    // Return true if all paths exist
    if sim_path.exists() && des_path.exists() && cont_path.exists() {
        true
    } else {
        false
    }
}

fn build_repository_file_struct(file_name: &str, file_path: &str) -> ProjectFile {
    let metadata = fs::metadata(file_path).unwrap();
    let modified_time = metadata.modified().unwrap();
    let mut file = ProjectFile {
        file_name: file_name.to_string(),
        file_path: file_path.to_string(),
        file_age: modified_time,
        exists: true,
    };
    file
}

fn build_project_file_struct(file_name: &str, file_path: &str) -> ProjectFile {
    println!("{}", file_path);
    
    let metadata = fs::metadata(file_path).unwrap();
    let modified_time = metadata.modified().unwrap();
    let mut file = ProjectFile {
        file_name: file_name.to_string(),
        file_path: file_path.to_string(),
        file_age: modified_time,
        exists: true,
    };
    file
}

fn check_file_has_changed(file: &ProjectFile) -> bool {
    let metadata = fs::metadata(file.file_path.clone()).unwrap();
    let modified_time = metadata.modified().unwrap();
    //println!("File age: {:?}", file.file_age);
    //println!("Modified time: {:?}", modified_time);
    
    if modified_time != file.file_age {
        //print!("File has changed!");
        true
    } else {
        //println!("File has not changed!");
        false
    }
}

fn check_file_exists(file: ProjectFile) -> bool {
    let metadata = fs::metadata(file.file_path);
    match metadata {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn cache_files_directory_from_repository(repo_path: &str, vivado_project_path: &str) -> FileStructure {
    // Throw error if *.xpr file does not exist
    let xpr_file_path = Path::new(vivado_project_path);
    if !xpr_file_path.exists() {
        panic!("The vivado project file does not exist");
    }
    
    // Create symbolic structures of three types of files
    let mut design_symbolic_list_vector: Vec<SymbolicLinkStructFile> = Vec::new();
    let mut simulation_symbolic_list_vector: Vec<SymbolicLinkStructFile> = Vec::new();
    let mut constraint_symbolic_list_vector: Vec<SymbolicLinkStructFile> = Vec::new();
    
    // Find the .srcs directory in the vivado project
    let vivado_project_srcs_path = Path::new(vivado_project_path);
    let vivado_project_srcs = fs::read_dir(vivado_project_srcs_path).unwrap();
    let mut srcs_path = String::new();
    for src in vivado_project_srcs {
        let src = src.unwrap();
        let src_name = src.file_name().into_string().unwrap();
        // println!("{}", src_name);
        if src_name.contains(".srcs") {
            srcs_path = src.path().into_os_string().into_string().unwrap();
        }
    }
    println!(".srcs file path: {}", srcs_path);
    
    // Create vivado project path
    let burrowed_vivado_design_path:&str = "/sources_1/new/design_sources"; 
    let burrowed_vivado_simulation_path:&str = "/sim_1/new/simulation_sources";
    let burrowed_vivado_constraint_path:&str = "/constrs_1/new/constraint_sources";
    
    let mut vivado_design_path = srcs_path.to_owned();
    let mut vivado_simulation_path = srcs_path.to_owned();
    let mut vivado_constraint_path = srcs_path.to_owned();
    
    vivado_design_path.push_str(burrowed_vivado_design_path);
    println!("{}", vivado_design_path);
    vivado_simulation_path.push_str(burrowed_vivado_simulation_path);
    println!("{}", vivado_simulation_path);
    vivado_constraint_path.push_str(burrowed_vivado_constraint_path);
    println!("{}", vivado_constraint_path);

    // Create design source structure
    let mut owned_design_repo_path = repo_path.to_owned();
    let burrowed_design_repo_path:&str = "/design_sources";
    //let final_design_repo_path:String = owned_design_repo_path.push_str(burrowed_design_repo_path);
    owned_design_repo_path.push_str(burrowed_design_repo_path);
    // Print the new path
    println!("{}", owned_design_repo_path);
    // Read the design_sources directory and list all files onto RepositoryFile struct
    let design_sources = fs::read_dir(owned_design_repo_path).unwrap();
    for file in design_sources {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_path = file.path().into_os_string().into_string().unwrap();
        vivado_design_path.push_str("/");
        vivado_design_path.push_str(&file_name);
        let mut repo_file = build_repository_file_struct(&file_name, &file_path);
        let mut project_file = build_project_file_struct(&file_name, &vivado_design_path);
        let mut symbolic_link = SymbolicLinkStructFile {
            repo_file,
            project_file,
            is_linked: true,
            source_type: "design".to_string(),
        };
        design_symbolic_list_vector.push(symbolic_link);
    };

    //Do the same for simulation_sources
    let mut owned_simulation_repo_path = repo_path.to_owned();
    let burrowed_simulation_repo_path:&str = "/simulation_sources";
    owned_simulation_repo_path.push_str(burrowed_simulation_repo_path);
    println!("{}", owned_simulation_repo_path);
    let simulation_sources = fs::read_dir(owned_simulation_repo_path).unwrap();
    for file in simulation_sources {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_path = file.path().into_os_string().into_string().unwrap();
        vivado_simulation_path.push_str("/");
        vivado_simulation_path.push_str(&file_name);
        let mut repo_file = build_repository_file_struct(&file_name, &file_path);
        let mut project_file = build_project_file_struct(&file_name, &vivado_simulation_path);
        let mut symbolic_link = SymbolicLinkStructFile {
            repo_file,
            project_file,
            is_linked: true,
            source_type: "simulation".to_string(),
        };
        simulation_symbolic_list_vector.push(symbolic_link);
    };
    
    // Do the same for constraint_sources
    let mut owned_constraint_repo_path = repo_path.to_owned();
    let burrowed_constraint_repo_path:&str = "/constraint_sources";
    owned_constraint_repo_path.push_str(burrowed_constraint_repo_path);
    println!("{}", owned_constraint_repo_path);
    let constraint_sources = fs::read_dir(owned_constraint_repo_path).unwrap();
    for file in constraint_sources {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_path = file.path().into_os_string().into_string().unwrap();
        vivado_constraint_path.push_str("/");
        vivado_constraint_path.push_str(&file_name);
        let mut repo_file = build_repository_file_struct(&file_name, &file_path);
        let mut project_file = build_project_file_struct(&file_name, &vivado_constraint_path);
        let mut symbolic_link = SymbolicLinkStructFile {
            repo_file,
            project_file,
            is_linked: true,
            source_type: "constraint".to_string(),
        };
        constraint_symbolic_list_vector.push(symbolic_link);
    };
    
    // Create the final FileStructure struct
    let mut file_structure = FileStructure {
        design: design_symbolic_list_vector,
        constraint: constraint_symbolic_list_vector,
        simulation: simulation_symbolic_list_vector,
    };
    file_structure
    
}


fn check_if_file_updated(file_name: &str, old_file_age: SystemTime) -> bool {
    let metadata = fs::metadata(file_name).unwrap();
    let modified_time = metadata.modified().unwrap();
    if modified_time > old_file_age {
        true
    } else {
        false
    }

}

fn modify_project_file(file: ProjectFile) -> ProjectFile {
    let metadata = fs::metadata(file.file_path.clone()).unwrap();
    let modified_time = metadata.modified().unwrap();
    let new_file = ProjectFile {
        file_name: file.file_name,
        file_path: file.file_path,
        file_age: modified_time,
        exists: true,
    };
    new_file
}


fn check_rewrite_file_loop(file_system: &mut FileStructure) {
    for mut design_file in &mut file_system.design {
        // Check from the repository if the file has changed
        if check_file_has_changed(&design_file.repo_file) {
            println!("Design file has changed from repository!");
            overwrite_file(&mut design_file.repo_file, &mut design_file.project_file);
        }
        // Check from the project if the file has changed
        else if check_file_has_changed(&design_file.project_file) {
            println!("Design file has changed from project!");
            overwrite_file(&mut design_file.project_file, &mut design_file.repo_file);
        }
        else{
            //println!("Design file has not changed!");
        }
    }
    for mut simulation_file in &mut file_system.simulation {
        // Check from the repository if the file has changed
        if check_file_has_changed(&simulation_file.repo_file) {
            overwrite_file(&mut simulation_file.repo_file, &mut simulation_file.project_file);
        }
        // Check from the project if the file has changed
        else if check_file_has_changed(&simulation_file.project_file) {
            overwrite_file(&mut simulation_file.project_file, &mut simulation_file.repo_file);
        }
    }
    for mut constraint_file in &mut file_system.constraint {
        // Check from the repository if the file has changed
        if check_file_has_changed(&constraint_file.repo_file) {
            overwrite_file(&mut constraint_file.repo_file, &mut constraint_file.project_file);
        }
        // Check from the project if the file has changed
        else if check_file_has_changed(&constraint_file.project_file) {
            overwrite_file(&mut constraint_file.project_file, &mut constraint_file.repo_file);
        }
        else { 
            //println!("Constraint file has not changed!");
        }
    }
}    

pub(crate) fn main_loop(repo_path: &str, vivado_project_path: &str, update_rate_ms: u64) {
    let mut file_structure = cache_files_directory_from_repository(repo_path, vivado_project_path);
    println!("File structure has been cached and is now ready to real time update!");
    println!("The real-time update rate is: {} ms", update_rate_ms);
    loop {
        check_rewrite_file_loop(&mut file_structure);
        std::thread::sleep(std::time::Duration::from_millis(update_rate_ms));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_if_sources_exist() {
        let sources_exist = check_if_sources_exist();
        assert_eq!(sources_exist, true);
    }
    #[test]
    fn test_build_repository_file_struct() {
        let file_name = "test_file.txt";
        let file_path = "test_file.txt";
        let file = build_repository_file_struct(file_name, file_path);
        assert_eq!(file.file_name, file_name);
        assert_eq!(file.file_path, file_path);
    }
    #[test]
    fn test_build_project_file_struct() {
        let file_name = "test_file.txt";
        let file_path = "test_file.txt";
        let file = build_project_file_struct(file_name, file_path);
        assert_eq!(file.file_name, file_name);
        assert_eq!(file.file_path, file_path);
    }

    #[test]
    fn test_check_file_exists() {
        let file_name = "test_file.txt";
        let file = fs::File::create(file_name).unwrap();
        let file = ProjectFile {
            file_name: file_name.to_string(),
            file_path: file_name.to_string(),
            file_age: SystemTime::now(),
            exists: true,
        };
        let exists = check_file_exists(file);
        assert_eq!(exists, true);
    }
    #[test]
    fn test_cache_files_directory_from_repository() {
        let repo_path = "..";
        let vivado_project_path = "../test_vivado_project/dummy_template";
        let file_structure = cache_files_directory_from_repository(repo_path, vivado_project_path);
        assert_eq!(file_structure.design.len(), 1);
        assert_eq!(file_structure.simulation.len(), 1);
        assert_eq!(file_structure.constraint.len(), 1);

        println!("Design files:");
        // Print the design files
        for design_file in file_structure.design {
            println!("{}", design_file.repo_file.file_name);
            println!("{}", design_file.repo_file.file_path);
            println!("{}", design_file.project_file.file_name);
            println!("{}", design_file.project_file.file_path);
        }
    }
    #[test]
    fn main_loop_test() {
        let repo_path = "..";
        let vivado_project_path = "../test_vivado_project/dummy_template";
        let update_rate_ns = 10;
        main_loop(repo_path, vivado_project_path, update_rate_ns);
    }
}