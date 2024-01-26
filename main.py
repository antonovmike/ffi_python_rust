import ctypes


def read_file(filename):
    with open(filename, 'r') as file:
        data = file.read()
    return data


filename = "example.txt"
text = read_file(filename)
rustlib = ctypes.cdll.LoadLibrary("target/release/librust_ffi.so")
rustlib.print_text(text.encode("utf-8"))
