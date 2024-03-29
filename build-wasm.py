import os
os.chdir("./wasm")
os.system('cmd /c "wasm-pack build --target web"')

with open("./pkg/priede_wasm.js", 'r') as file:
    existing_content = file.read()

    # Combine the new text with the existing content
    new_content = "import { wasm_print } from \"../App.js\";\n" + existing_content

    # Write the combined content back to the file
    with open("./pkg/priede_wasm.js", 'w') as file:
        file.write(new_content)
            

os.system('cmd /c "move ./pkg ../web-editor/src/"')