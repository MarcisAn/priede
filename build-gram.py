import os
os.chdir("./interpreter/src/generator/hime")
os.system('cmd /c "himecc.exe priede.gram -t:rust"')