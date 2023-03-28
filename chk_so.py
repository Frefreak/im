import ctypes

d = ctypes.CDLL("./imrs/target/debug/libimrs.so")
# d = ctypes.CDLL("/usr/lib/gtk-3.0/3.0.0/immodules/im-fcitx5.so")
im_module_list = d["im_module_list"]


class GtkIMContextInfo(ctypes.Structure):
    _fields_ = [
        ("context_id", ctypes.c_char_p),
        ("context_name", ctypes.c_char_p),
        ("domain", ctypes.c_char_p),
        ("domain_dirname", ctypes.c_char_p),
        ("default_locales", ctypes.c_char_p),
    ]


im_module_list.argtypes = [
    ctypes.POINTER(ctypes.POINTER(ctypes.POINTER(GtkIMContextInfo))),
    ctypes.POINTER(ctypes.c_uint),
]
im_module_list.restype = None

ptr = ctypes.c_void_p(0)
ptr = ctypes.cast(ptr, ctypes.POINTER(ctypes.POINTER(GtkIMContextInfo)))
num = ctypes.c_uint(0)

im_module_list(ctypes.pointer(ptr), ctypes.pointer(num))
info = ptr.contents.contents

num = num.value
infos = ptr[:num]

for info in infos:
    print('-----------------')
    contents = info.contents
    print(contents.context_id)
    print(contents.context_name)
    print(contents.domain)
    print(contents.domain_dirname)
    print(contents.default_locales)
