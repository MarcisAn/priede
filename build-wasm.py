import os
os.chdir("./wasm")
os.system('cmd /c "wasm-pack build --target web"')
os.system('cmd /c "move ./pkg ../online-editor/pkg"')