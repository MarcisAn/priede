import os

def build_wasm():
    os.chdir("./wasm")
    os.system('cmd /c "wasm-pack build --target web"')

    with open("./pkg/priede_wasm.js", 'r') as file:
        existing_content = file.read()

        # Combine the new text with the existing content
        new_content = "import { wasm_print, wasm_input, get_stumbrs_data, code_replace } from \"../priede.js\";\n" + existing_content

        # Write the combined content back to the file
        with open("./pkg/priede_wasm.js", 'w') as file:
            file.write(new_content)

    if os.name == 'nt':   
        os. remove("./pkg/.gitignore")
        os.system('cmd /c  "robocopy  ./pkg ../web-editor-new/src/pkg" /E /MOVE /NFL /NDL /NJH /NJS /nc /ns /np')
    else:
        print("wasm package moving to web-editor not implemented on other platforms")
        
if __name__ == "__main__":
    build_wasm()
