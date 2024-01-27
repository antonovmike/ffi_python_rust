# ffi_python_rust
Just a ffi test

This project consists of two parts: a Python 3 script that opens the file "example.txt" and reads the text from it, and a Rust script that receives the read data and prints it to the terminal. 
To run this project, you need to follow these steps:

1. Install Python 3 and Rust on your computer.
2. Clone the project files to a folder on your computer using the command ```git clone https://github.com/antonovmike/ffi_python_rust.git```. You should have three files: `main.py`, `src/lib.rs` and `Cargo.toml`.
3. Open the terminal or command line on your computer and go to the folder with the project files. You can use the command cd to change the directory. For example, if your folder is called project and is located on the desktop, you can enter ```cd ffi_python_rust```.
4. Run the command ```cargo build --release``` in the terminal to compile the Rust script into a dynamic library that can be loaded from Python. This will create a file `rustlib.so` in the `target/release folder`.
5. Run the command ```python3 main.py``` in the terminal to run the Python script.

If all the steps are done correctly, you should see the text "Hello world" in the terminal. Congratulations, you have successfully run the project!
