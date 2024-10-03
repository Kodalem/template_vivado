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
    design_path: String,
    constraint: Vec<SymbolicLinkStructFile>,
    constraint_path: String,
    simulation: Vec<SymbolicLinkStructFile>,
    simulation_path: String,
    project_path: String,
    repo_path: String,
}


fn overwrite_file(from_file: &mut ProjectFile, to_file: &mut ProjectFile) -> bool {
    // Save the old file age
    let old_file_age = from_file.file_age;


    let from_file_path = Path::new(&from_file.file_path);
    println!("From file: {}", from_file.file_path);
    let to_file_path = Path::new(&to_file.file_path);
    println!("To file:{}", to_file.file_path);
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
    // If the file ends with tilde, then it is a backup file - IGNORE IT
    if file_name.ends_with("~") {
        return ProjectFile {
            file_name: file_name.to_string(),
            file_path: file_path.to_string(),
            file_age: SystemTime::now(),
            exists: false,
        };
    }

    println!("File name: {}", file_name);
    println!("File path: {}", file_path);

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
    // Return false if the file does not exist
    if file.file_name.ends_with("~") {
        return false;
    }
    //println!("File path: {}", file.file_path);
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

fn return_folder_file_vector(folder_path: &str) -> Vec<String> {
    let folder_path = Path::new(folder_path);
    let folder = fs::read_dir(folder_path).unwrap();
    let mut file_vector: Vec<String> = Vec::new();
    for file in folder {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        file_vector.push(file_name);
    }
    file_vector
}

fn return_missing_file_names_from_vector(file_vector: Vec<String>, mut symbolic_link_vector: &Vec<SymbolicLinkStructFile>) -> Vec<String> {
    let mut missing_files: Vec<String> = Vec::new();
    for file in file_vector {
        let mut file_exists = false;
        for symbolic_link in symbolic_link_vector.clone() {
            if symbolic_link.repo_file.file_name == file {
                file_exists = true;
            }
        }
        if !file_exists {
            missing_files.push(file);
        }
    }
    missing_files
}

fn check_file_structure(file_struct: &mut FileStructure) -> bool {
    // Check how many files are in the directories
    let design_files = return_folder_file_vector(&mut file_struct.design_path);
    let design_file_amount = design_files.len() as u32;

    let sim_files = return_folder_file_vector(&mut file_struct.simulation_path);
    let simulation_file_amount = sim_files.len() as u32;

    let constraint_files = return_folder_file_vector(&mut file_struct.constraint_path);
    let constraint_file_amount = constraint_files.len() as u32;

    // Compare the amount of files in the design_sources directory with the amount of files in the design vector
    if design_file_amount != file_struct.design.len() as u32 {
        let missing_files = return_missing_file_names_from_vector(design_files, &mut file_struct.design);
          // Add missing files to the repository
          for missing_file in missing_files.clone(){
              // Build the file structure
              let mut file_path_correction = file_struct.design_path.clone();
                file_path_correction.push_str("/");
                file_path_correction.push_str(&missing_file);
              let project_struct = build_project_file_struct(&missing_file, &file_path_correction);
              // Add the file to the repository
              let repo_struct = add_file_from_vivado_to_repository(project_struct.clone(), &file_struct.repo_path, "design");
              // Build the repo file structure
              update_symbolic_link_list_vector(file_struct, repo_struct, project_struct, "design");
              // Print the new symbolic link list vector
              println!("Design symbolic link list vector:");
                for design_file in &file_struct.design {
                    println!("{}", design_file.repo_file.file_name);
                }
          }
        // If missing file is null, then return true
        if missing_files.is_empty() {
            // Holy shit how backup files are obnoxious
            return true;
        }
        return false;
    }
    // Compare the amount of files in the simulation_sources directory with the amount of files in the simulation vector
    if simulation_file_amount != file_struct.simulation.len() as u32 {
        let missing_files = return_missing_file_names_from_vector(sim_files, &mut file_struct.simulation);
        // Add missing files to the repository
        for missing_file in missing_files.clone(){
            // Build the file structure
            let mut file_path_correction = file_struct.simulation_path.clone();
            file_path_correction.push_str("/");
            file_path_correction.push_str(&missing_file);
            let project_struct = build_project_file_struct(&missing_file, &file_path_correction);
            // Add the file to the repository
            let repo_struct = add_file_from_vivado_to_repository(project_struct.clone(), &file_struct.repo_path, "simulation");
            // Build the repo file structure
            update_symbolic_link_list_vector(file_struct, repo_struct, project_struct, "simulation");
            println!("Design symbolic link list vector:");
            for design_file in &file_struct.simulation {
                println!("{}", design_file.repo_file.file_name);
            }
        }
        // If missing file is null, then return true
        if missing_files.is_empty() {
            return true;
        }
        return false;
    }
    // Compare the amount of files in the constraint_sources directory with the amount of files in the constraint vector
    if constraint_file_amount != file_struct.constraint.len() as u32 {
        let missing_files = return_missing_file_names_from_vector(constraint_files, &mut file_struct.constraint);
        // Add missing files to the repository
        for missing_file in missing_files.clone(){
            // Build the file structure
            let mut file_path_correction = file_struct.constraint_path.clone();
            file_path_correction.push_str("/");
            file_path_correction.push_str(&missing_file);
            let project_struct = build_project_file_struct(&missing_file, &file_path_correction);
            // Add the file to the repository
            let repo_struct = add_file_from_vivado_to_repository(project_struct.clone(), &file_struct.repo_path, "constraint");
            // Build the repo file structure
            update_symbolic_link_list_vector(file_struct, repo_struct, project_struct, "constraint");
            println!("Design symbolic link list vector:");
            for design_file in &file_struct.constraint {
                println!("{}", design_file.repo_file.file_name);
            }
        }
        // If missing file is null, then return true
        if missing_files.is_empty() {
            return true;
        }
        return false;
    }
    true
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
        // Clone the design path string
        let mut vivado_design_path_loop = vivado_design_path.clone();
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_path = file.path().into_os_string().into_string().unwrap();
        vivado_design_path_loop.push_str("/");
        vivado_design_path_loop.push_str(&file_name);
        let mut repo_file = build_repository_file_struct(&file_name, &file_path);
        let mut project_file = build_project_file_struct(&file_name, &vivado_design_path_loop);
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
        // Clone the design path string
        let mut vivado_simulation_path_loop = vivado_simulation_path.clone();
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_path = file.path().into_os_string().into_string().unwrap();
        vivado_simulation_path_loop.push_str("/");
        vivado_simulation_path_loop.push_str(&file_name);
        let mut repo_file = build_repository_file_struct(&file_name, &file_path);
        let mut project_file = build_project_file_struct(&file_name, &vivado_simulation_path_loop);
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
        // Clone constraint path string
        let mut vivado_constraint_path_loop = vivado_constraint_path.clone();
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        let file_path = file.path().into_os_string().into_string().unwrap();
        vivado_constraint_path_loop.push_str("/");
        vivado_constraint_path_loop.push_str(&file_name);
        let mut repo_file = build_repository_file_struct(&file_name, &file_path);
        let mut project_file = build_project_file_struct(&file_name, &vivado_constraint_path_loop);
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
        repo_path: repo_path.to_string(),
        project_path: vivado_project_path.to_string(),
        design: design_symbolic_list_vector,
        design_path: vivado_design_path,
        constraint: constraint_symbolic_list_vector,
        constraint_path: vivado_constraint_path,
        simulation: simulation_symbolic_list_vector,
        simulation_path: vivado_simulation_path,
    };
    file_structure

}

fn update_symbolic_link_list_vector (file_structure: &mut FileStructure, repo_file_append: ProjectFile,
                                     project_file_append: ProjectFile, source_type: &str) {
    let mut symbolic_link = SymbolicLinkStructFile {
        repo_file: repo_file_append,
        project_file: project_file_append,
        is_linked: true,
        source_type: source_type.to_string(),
    };
    match source_type {
        "design" => file_structure.design.push(symbolic_link),
        "simulation" => file_structure.simulation.push(symbolic_link),
        "constraint" => file_structure.constraint.push(symbolic_link),
        _ => panic!("The source type does not exist"),
    }

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

fn add_file_from_vivado_to_repository(file: ProjectFile, repo_path: &str, source_type: &str) -> ProjectFile {
    // Ignore the files that end with tilde
    if file.file_name.ends_with("~") {
        return ProjectFile {
            file_name: file.file_name,
            file_path: file.file_path,
            file_age: SystemTime::now(),
            exists: false,
        };
    }
    let mut owned_repo_path = repo_path.to_owned();
    owned_repo_path.push_str("/");


    match source_type{
        "design" => owned_repo_path.push_str("design_sources/"),
        "simulation" => owned_repo_path.push_str("simulation_sources/"),
        "constraint" => owned_repo_path.push_str("constraint_sources/"),
        _ => panic!("The source type does not exist"),
    }
    owned_repo_path.push_str(&file.file_name);;
    println!("{}", owned_repo_path);
    let vivado_path_name = file.file_path.to_owned();
    println!("{}", vivado_path_name);

    fs::copy(vivado_path_name, owned_repo_path.clone()).expect("Something went wrong while copying the file");
    // Copy the file from the vivado project to the repository
    // Check if the file exists
    let metadata = fs::metadata(owned_repo_path.clone()).unwrap();
    let modified_time = metadata.modified().unwrap();
    println!("{:?}", modified_time);

    println!("{}", owned_repo_path);

    let repo_struct = build_repository_file_struct(&file.file_name, &owned_repo_path);


    repo_struct
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
        if check_file_structure(&mut file_structure) {
            continue
        } else {
            println!("The file structure has changed!");
        }
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