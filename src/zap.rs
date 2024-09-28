use std::{fs, env, path::PathBuf};

pub fn create_project(project_name: &str) {
    let project_path = PathBuf::from(project_name);
    fs::create_dir(project_path.clone()).unwrap();
    fs::create_dir(project_path.join("src")).unwrap();
    fs::create_dir(project_path.join("test")).unwrap();
    fs::File::create_new(project_path.join("src/main.py")).unwrap();
    fs::write(project_path.join("src/main.py"), "import random\n\ndef fib(n):\n    a, b = 0, 1 \n    for _ in range(n):\n        print(a, end='')\n        a, b = b, a+b\n    print()\nmax_terms = min(random.randint(1, 100), 500)\nfib(max_terms)").unwrap();
    fs::write(project_path.join("requirements.txt"), "# Add your dependencies here!").unwrap();
    fs::write(project_path.join("test_main.py"), "# import the package\nimport main\n\n# import the main module\nfrom main import main\n\n# or an object inside the main module\nfrom main.main import my_object").unwrap();
    println!("Your '{}' project has been created!", project_name);
}

pub fn run_project() {
    let current_dir = env::current_dir().unwrap();
    let project_path = current_dir.join("zap");
    println!("Running project in {}", project_path.display());
}

pub fn test_project() {
    let current_dir = env::current_dir().unwrap();
    let project_path = current_dir.join("zap");
    println!("Running tests in {}", project_path.display());
}