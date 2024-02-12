#!/usr/bin/env python3
'''
在构建每个程序前替换链接脚本中的初始地址
'''

import os

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0
apps = os.listdir('src/bin')
apps.sort()
for app in apps:
    app = app[:app.find('.')]
    lines = []
    lines_before = []
    with open(linker, 'r') as f:
        for line in f.readlines():
            lines_before.append(line)
            line = line.replace(hex(base_address), hex(base_address+step*app_id))
            lines.append(line)
            pass
        pass
    with open(linker, 'w+') as f:
        f.writelines(lines)
        pass
    os.system(f'cargo build --bin {app} --release')
    print(f'[build.py] application {app} start with address {hex(base_address+step*app_id)}')
    with open(linker, 'w+') as f:
        f.writelines(lines_before)
        pass
    app_id += 1
    pass
