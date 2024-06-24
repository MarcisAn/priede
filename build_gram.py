import os
os.chdir("./interpreter/src/hime")
os.system('cmd /c "himecc.exe priede.gram -t rust"')

with open("priede.rs", 'r', encoding="utf-8") as file:
        existing_content = file.read()

        # Combine the new text with the existing content
        new_content = "#![allow(unused_variables)]\n" + existing_content

        # Write the combined content back to the file
        with open("priede.rs", 'w', encoding="utf-8") as file:
            file.write(new_content)