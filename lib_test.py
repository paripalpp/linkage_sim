import ctypes

# ScissorDimension has 4 length
# a : length of the element right up to the right
# b : length of the element right up to the left
# c : distance from a origin to cross point of a and b
# d : distance from b origin to cross point of a and b
class ScissorDimension(ctypes.Structure) :
    _fields_ = [("a", ctypes.c_double),
                ("b", ctypes.c_double),
                ("c", ctypes.c_double),
                ("d", ctypes.c_double)]

# 共有ライブラリの読み込み
linkage_sim = ctypes.CDLL('./target/release/liblinkage_sim.so')  # Linux の場合

# create_sizzor_dimension_array 定義
create_sizzor_dimension_array = linkage_sim.create_sizzor_dimension_array
create_sizzor_dimension_array.argtypes = [ctypes.c_size_t]
create_sizzor_dimension_array.restype = ctypes.POINTER(ScissorDimension)

get_sizzor_dimension_array_element = linkage_sim. get_sizzor_dimension_array_element
get_sizzor_dimension_array_element.argtypes = [ctypes.POINTER(ScissorDimension), ctypes.c_size_t]
get_sizzor_dimension_array_element.restype = ScissorDimension

set_sizzor_dimension_array_element = linkage_sim. set_sizzor_dimension_array_element
set_sizzor_dimension_array_element.argtypes = [ctypes.POINTER(ScissorDimension), ctypes.c_size_t, ScissorDimension]

# Rustの関数を呼び出し
result = linkage_sim.add(2, 3)
print(f"Result from Rust: {result}")

array_size = 5
array = create_sizzor_dimension_array(array_size)
for i in range(array_size):
    element = get_sizzor_dimension_array_element(array, i)
    print(f"element {i} : a={element.a}, b={element.b}, c={element.c}, d={element.d}")
second_scissor = ScissorDimension(1.0, 1.0, 0.6, 0.4)
print(f"types : array={type(array)}, 1={type(1)}, second_scissor={type(second_scissor)}")
set_sizzor_dimension_array_element(array, 1, second_scissor)
for i in range(array_size):
    element = get_sizzor_dimension_array_element(array, i)
    print(f"element {i} : a={element.a}, b={element.b}, c={element.c}, d={element.d}")


# linkage_sim.run_scissor_test()
# linkage_sim.run_triangle_test()
# linkage_sim.run_crosslink_test()


# 注意: Rustで確保されたメモリを解放する必要があります
ctypes.CDLL(None).free(array)