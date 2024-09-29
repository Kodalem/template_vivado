mod function_library;
mod xml_manipulation;

use std::env;

fn main() {
    // Run the implementation
    //let repo_path = "..";
    //let vivado_project_path = "../test_vivado_project/dummy_template";
    //let update_rate_ns = 1000;
    
    let args: Vec<String> = env::args().collect();
    let repo_path = &args[1];
    let vivado_project_path = &args[2];
    let update_rate_ns_string = &args[3];
    
    // Convert the string to an integer
    let update_rate_ns: u64 = update_rate_ns_string.parse().unwrap();
    
    function_library::main_loop(repo_path, vivado_project_path, update_rate_ns);
}


                   
                   
                   
                   
