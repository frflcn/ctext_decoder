import requests
import re
import os

URL = "https://www.unicode.org/Public/MAPPINGS/OBSOLETE/EASTASIA/KSC/KSC5601.TXT"
REPLACEMENT_CHARACTER = 65533

response = requests.get(URL)

if response.status_code != 200:
    print(URL)
    print(response.status_code)
    print(response.reason)

content = response.text

lines = content.split('\n')
ks_encoding = [[REPLACEMENT_CHARACTER] * 94 for i in range(94)]
start_line = 0
regex = re.compile("^(0x[0-9A-Fa-f]{4}\t){2}")
for x in range(len(lines)):
    if regex.match(lines[x]):
        hex_codes = lines[x].split('\t')
        first_byte = int(hex_codes[0][2:4], 16) - 161
        second_byte = int(hex_codes[0][4:6], 16) - 161
        if first_byte < 0 or first_byte >= 94:
            continue
        if second_byte < 0 or second_byte >= 94:
            continue
        
        unicode = int(hex_codes[1], 16)
        ks_encoding[first_byte][second_byte] = unicode

encoding_file = f"// Generated from: {URL}\n\n\n"
encoding_file += f"pub const KS_C5601: [[u32; {len(ks_encoding[0])}]; 94] = \n["
for x in range(94):
    encoding_file += "["
    for a in range(6):
        encoding_file += "\n"
        for b in range(16):
            if (a == 0 and b == 0) or (a == 5 and b == 15):
                encoding_file += " " * 7
            else:
                num_str = str(ks_encoding[x][(a * 16) + b - 1])
                num_spaces = 5 - len(num_str)
                encoding_file += (" " * num_spaces) + num_str + ", "

    encoding_file += "],\n"
encoding_file += "];"
file_directory = os.path.dirname(os.path.realpath(__file__))
encoding_path = os.path.join(file_directory, "../ks_c5601.rs")
with open(encoding_path, "w") as f:
    f.write(str(encoding_file))

webpage_path = os.path.join(file_directory, "webpages/ks_c5601.txt")
with open(webpage_path, "w") as f:
    f.write(content)