import ctypes


rustlib = ctypes.cdll.LoadLibrary("target/release/librust_ffi.so")
get_hello_world = rustlib.get_hello_world
get_hello_world.restype = ctypes.c_char_p

hello_world = get_hello_world()
print(hello_world)
