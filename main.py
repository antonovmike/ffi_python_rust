import ctypes


name = input("Enter your name: ")
birth_year = input("Enter your birth year: ")

rustlib = ctypes.cdll.LoadLibrary("target/release/librust_ffi.so")
rustlib.print_person(name.encode("utf-8"), birth_year.encode("utf-8"))
