# Zap

## What is Zap?

Zap is a simple project manager for Python built with Rust, designed to help you quickly create and run your Python projects. It is inspired by the popular project managers like `cargo` and `npm`.

## How to use Zap?

To create a new project, run the following command in your terminal:

```
zap new
```

Then it will ask you to enter a project name. After entering the project name, Zap will create a new directory with the project name and create a `src` and `test` directory inside it. It will also create a `main.py` file inside the `src` directory and a `test_main.py` file inside the `test` directory. Finally, it will create a `requirements.txt` file inside the project directory.

To run your project, run the following command in your terminal:

```
zap run
```

Zap will run your project by executing the `main.py` file in the `src` directory. If you want to run the tests, you can run the following command:

```
zap test
```

Zap will run the tests in the `test` directory.

## Features

- Create and run Python projects
- Run tests
- Supports Python 3.x

## Contributing

Contributions are welcome! If you find any bugs or have any suggestions, please open an issue or submit a pull request.

## License

This project is licensed under the APACHE LICENSE 2.0. See the [LICENSE](LICENSE) file for more information.
