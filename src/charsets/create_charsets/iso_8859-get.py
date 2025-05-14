import requests
import re
import os

REPLACEMENT_CHARACTER = 65533
URL = "https://www.rfc-editor.org/rfc/rfc1345.txt"
response = requests.get(URL)

if response.status_code != 200:
    print(URL)
    print(response.status_code)
    print(response.reason)

content = response.text

character_mnemonic_start = 0
character_mnemonic_end = 0
charsets_start = 0
charsets_end = 0

lines = content.split("\n")

for idx, line in enumerate(lines):
    if line == "3.  CHARACTER MNEMONIC TABLE":
        character_mnemonic_start = idx
    if line == "4.  CHARSETS":
        character_mnemonic_end = idx
    if line == "5.  CHARSET TABLES":
        charsets_start = idx
    if line == "ACKNOWLEDGEMENTS":
        charsets_end = idx

regex_discard = re.compile("(^Simonsen)|(^\x0c)|(RFC 1345)|(^$)")
regex_mnemonic = re.compile("^ ([\\S]+)[ ]+([0-9a-fA-F]{4})[ ]+[\\S]+")

#mnemonic = re.compile("^ (\\S+)")
mnemonics = {}
mnemonics["??"] = REPLACEMENT_CHARACTER
for line in lines[character_mnemonic_start:character_mnemonic_end]:
    if regex_discard.match(line):
        #print(repr(line))
        continue
    if regex_mnemonic.match(line):
        codes = line.split()
        mnemonics[codes[0]] = int(codes[1], 16)

ISOs = []

for x in range(1, 10):
    regex_charset = re.compile(f"^  &charset ISO_8859-{x}")
    found_iso = False
    found_start = False
    iso = []
    for line in lines[charsets_start:charsets_end]:
        if found_start and re.compile("^  &charset").match(line):
            break
        if found_start and not regex_discard.match(line):
            characters = line.split()
            for character in characters:
                iso.append(mnemonics[character])
        if found_iso and line == "  &code 0":
            found_start = True
        if regex_charset.match(line):
            found_iso = True
    ISOs.append(iso)

file_directory = os.path.dirname(os.path.realpath(__file__))
file_name = f"../iso_8859.rs"

file_string = f"// Generated from: {URL}\n\n\n"

for idx, iso in enumerate(ISOs):
    file_string += f"pub const ISO_8859_{idx+1}: [u32; 256] = [\n"
    for idy in range(16):
        for idz in range(16):
            num_str = str(iso[(idy * 16) + idz])
            num_space = 5 - len(num_str)
            file_string += " " * num_space
            file_string += num_str + ", "
        file_string += "\n"
    file_string += "];\n\n"
file_string = file_string[:-2]




with open(os.path.join(file_directory, file_name), "w") as f:
    f.write(file_string)
