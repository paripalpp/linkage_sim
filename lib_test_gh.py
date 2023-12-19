# import rhinoscriptsyntax as rs
import sys
import ctypes
# import numpy as np

class ScissorDimension(ctypes.Structure):
    _fields_ = [("a", ctypes.c_double),
                ("b", ctypes.c_double),
                ("c", ctypes.c_double),
                ("d", ctypes.c_double)]

class line(ctypes.Structure):
    _fields_ = [("x1", ctypes.c_double),
                ("y1", ctypes.c_double),
                ("x2", ctypes.c_double),
                ("y2", ctypes.c_double)]

class solve_scissor_return(ctypes.Structure):
    _fields_ = [("error", ctypes.c_size_t),
                ("num_line", ctypes.c_size_t),
                ("lines", ctypes.POINTER(line))]

msvcrt = ctypes.windll.msvcrt
linkage_sim = ctypes.CDLL('C:/Users/Nakanishi/Documents/linkage_sim/target/release/linkage_sim.dll')

# create_scissor_dimension_array define
create_scissor_dimension_array = linkage_sim.create_scissor_dimension_array
create_scissor_dimension_array.argtypes = [ctypes.c_size_t]
create_scissor_dimension_array.restype = ctypes.POINTER(ScissorDimension)

get_scissor_dimension_array_element = linkage_sim.get_scissor_dimension_array_element
get_scissor_dimension_array_element.argtypes = [ctypes.POINTER(ScissorDimension), ctypes.c_size_t]
get_scissor_dimension_array_element.restype = ScissorDimension

set_scissor_dimension_array_element = linkage_sim.set_scissor_dimension_array_element
set_scissor_dimension_array_element.argtypes = [ctypes.POINTER(ScissorDimension), ctypes.c_size_t, ScissorDimension]

solve_from_scissor_dimension_array = linkage_sim.solve_from_scissor_dimension_array
solve_from_scissor_dimension_array.argtypes = [ctypes.POINTER(ScissorDimension), ctypes.c_size_t, ctypes.c_double, ctypes.c_double]
solve_from_scissor_dimension_array.restype = solve_scissor_return

# call Rust function
result = linkage_sim.add(2, 3)
print "Result from Rust: {}".format(result)

array_size = 5
array = create_scissor_dimension_array(array_size)
for i in range(array_size):
    element = get_scissor_dimension_array_element(array, i)
    print "element {} : a={}, b={}, c={}, d={}".format(i, element.a, element.b, element.c, element.d)

second_scissor = ScissorDimension(1.0, 1.0, 0.6, 0.4)
print "types : array={}, 1={}, second_scissor={}".format(type(array), type(1), type(second_scissor))
set_scissor_dimension_array_element(array, 1, second_scissor)
for i in range(array_size):
    element = get_scissor_dimension_array_element(array, i)
    print "element {} : a={}, b={}, c={}, d={}".format(i, element.a, element.b, element.c, element.d)

solve_return = solve_from_scissor_dimension_array(array, array_size, 0.8, 0.0)
if solve_return.error == 0:
    print "num_line = {}".format(solve_return.num_line)
    for i in range(solve_return.num_line):
        line = solve_return.lines[i]
        print ([line.x1, line.x2], [line.y1, line.y2])
# inputs = np.linspace(0.8, 0.5, 50)
# for i in inputs:
#     solve_return = solve_from_scissor_dimension_array(array, array_size, i, 0.0)
#     if solve_return.error == 0:
#         print (solve_return.lines[9].x2, solve_return.lines[9].y2)


# memory free
if sys.platform.startswith('linux'):
    ctypes.CDLL(None).free(array)
elif sys.platform.startswith('win'):
    msvcrt.free(array)
elif sys.platform.startswith('darwin'):
    ctypes.CDLL(None).free(array)
