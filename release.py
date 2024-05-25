from build_wasm import build_wasm
import json
import os
import time

def build(version):
    build_wasm()
    os.chdir("../cli")
    os.system('cargo build -r')
    os.mkdir("../releases/" + version)
    os.rename("./target/release/priede-cli.exe", os.path.join("..\\releases\\" + version + "\\", "priede.exe"))
    os.chdir("../launcher")
    os.system('cargo tauri build')
    os.rename("./src-tauri/target/release/launcher.exe", os.path.join("..\\releases\\" + version + "\\", "priede_launcher.exe"))
    
    
    

releases = json.load(open("releases.json"))
print(releases[-1])

new_version = input("Vai izveidot jaunu versiju? (0/1)")
if new_version == "0":
    build(releases[-1]["version"].replace(".", "-"))
else:
    version_number = releases[0]["version"].split(".")
    which_version_to_bump = input("Kuru versijas indexu palielināt (M/m/p)")
    if which_version_to_bump == "M":
        version_number[0] = int(version_number[0]) + 1
        version_number[1] = 0
        version_number[2] = 0
        
    elif which_version_to_bump == "m":
        version_number[1] = int(version_number[1]) + 1
        version_number[2] = 0
        
    elif which_version_to_bump == "p":
        version_number[2] = int(version_number[2]) + 1
    number_stringified = str(version_number[0]) + "." + str(version_number[1]) + "." + str(version_number[2])
    releases.append({"version": number_stringified, "release_notes": []})
    print(version_number)
    with open('releases.json', 'w') as f:
        json.dump(releases, f)
    build(str(version_number[0]) + "-" + str(version_number[1]) + "-" + str(version_number[2]))
        
    
    
