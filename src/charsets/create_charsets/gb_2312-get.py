import requests
from bs4 import BeautifulSoup
from bs4.element import Tag
import re
import sys
import os


def print_type(obj) -> None:
    print(type(obj).__name__)
    print(type(obj).__module__)

webpages = []

def find_and_operate_on_table(url: str, fn, is_first: bool) -> list:
    global webpages
    # if url != "https://icu4c-demos.unicode.org/icu-bin/convexp?conv=ibm-1383_P110-1999&b=A1&s=ALL#layout" and url != "https://icu4c-demos.unicode.org/icu-bin/convexp?conv=gb2312":
    #     return

    response = requests.get(url)
    if response.status_code != 200:
        print(response.reason)
        print(response.status_code)
        print(url)
        sys.exit()
    content = response.content
    
    if "conv=gb2312" in url:
        name = "master"
        webpages.append((name + ".html", response.text))
    else:
        locator = "1999&b="
        index = url.find(locator)
        name = url[index + len(locator): index + len(locator) + 2]
        webpages.append((f"{name[0]}/" + name + ".html", response.text))
    
    print(name)
    soup = BeautifulSoup(content, "html.parser")
    table = soup.find(attrs={"summary": re.compile("^A 16 by 16")})
    rows = table.find_all("tr")

    array = []
    row: Tag
    for idx, row in enumerate(rows[12:-1]):
        cells = row.find_all("td")
        cell: Tag
        for idy, cell in enumerate(cells):
            if (idx == 0 and idy == 0) or (idx == 5 and idy == 15):
                continue
            if "class" not in cell.attrs or cell["class"] != ["reserved"]:
                array.append(fn(cell))
            else:
                if not is_first:
                    array.append(int("FFFD", 16))
    return array

def get_url(tag: Tag) -> str:
    domain = "https://icu4c-demos.unicode.org"
    a = tag.find("a")
    subdomain = a["href"]
    return domain + subdomain

def operate_first_table(tag: Tag) -> list:
    url = get_url(tag)
    return find_and_operate_on_table(url, operate_second_table, False)

def operate_second_table(tag: Tag) -> None:
    span = tag.find("span")
    return int(span.text, 16)

URL = "https://icu4c-demos.unicode.org/icu-bin/convexp?conv=gb2312"


encoding = find_and_operate_on_table(URL, operate_first_table, True)


encoding_file = f"// Generated from: {URL}\n\n\n"
encoding_file += f"pub const GB_2312: [[u32; {len(encoding[0])}]; 94] = \n["
for x in range(94):
    encoding_file += "["
    for a in range(6):
        encoding_file += "\n"
        for b in range(16):
            if (a == 0 and b == 0) or (a == 5 and b == 15):
                encoding_file += " " * 7
            else:
                num_str = str(encoding[x][(a * 16) + b - 1])
                num_spaces = 5 - len(num_str)
                encoding_file += (" " * num_spaces) + num_str + ", "

    encoding_file += "],\n"
encoding_file += "];"
file_directory = os.path.dirname(os.path.realpath(__file__))
encoding_path = os.path.join(file_directory, "../gb_2312.rs")
with open(encoding_path, "w") as f:
    f.write(str(encoding_file))

for webpage in webpages:
    webpage_path = os.path.join(file_directory, "webpages/gb_2312/" + webpage[0])
    with open(webpage_path, "w") as f:
        f.write(webpage[1])





