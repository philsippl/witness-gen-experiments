import struct

HASHMAP_LEN = 256
WITNESS2SIG_LEN = 6
CONSTANTS_LEN = 2

with open(f"circuit/test_cpp/test.dat", "rb") as f:
    f.read(HASHMAP_LEN*24) # ignore
    f.read(WITNESS2SIG_LEN*8)  # ignore
    
    # we're only here for the constants:
    for _ in range(0,CONSTANTS_LEN):
        sv, typ, lv1, lv1, lv1, lv1 = struct.unpack('<iIQQQQ', bytearray(f.read(40)))
        is_long = bool(typ & 0x80000000)
        print(f"is_long = {is_long}, short_val = {sv}, type = {typ}, long_val = [{lv1}, {lv1}, {lv1}, {lv1}]")