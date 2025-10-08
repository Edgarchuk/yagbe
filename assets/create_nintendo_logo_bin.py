
#!/usr/bin/env python3
"""
create_bin.py

Writes a binary file with the following bytes (hex representation):

CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
"""

# ------------------------------------------------------------------
# 1. Prepare the raw data as a list of integers (each in 0x00–0xFF).
#    You can copy‑paste the hex values directly, or type them manually.
# ------------------------------------------------------------------
hex_values = """
CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
"""

# Convert the string into a list of integers.
bytes_list = [int(b, 16) for b in hex_values.split()]

# ------------------------------------------------------------------
# 2. Write the data to a binary file.
#    The filename can be changed if you want something else.
# ------------------------------------------------------------------
output_path = "nintendo.bin"

with open(output_path, "wb") as f:
    f.write(bytearray(bytes_list))

print(f"✅ Written {len(bytes_list)} bytes to '{output_path}'.")
