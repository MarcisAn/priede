import os
os.chdir("./compiler/src/hime")
os.system('cmd /c "himecc.exe priede.gram -t rust"')