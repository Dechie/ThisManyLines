# ThisManyLines
A Rust command line app that shows you how many lines of code you wrote today :)

this app will take two main modes: whole directory tree and specific folder.

whole directory mode: this will traverse your whole file system, and find recent files
that you have written. for this you will have to specify which language you have used.

therefore it will take a second argument as the language/framework name.

specific folder mode: this will work on a folder that you will specify for it. it will be aimed at analyzing the project you're recently working on.

 ##usage:

    ```
    manylines [OPTION]

    -s --specific <specific> analyze a specific directory
    ```

so the command would be like: 

 ```-w rust``` ==> whole file sytem, targetting rust project
 ```-s /path/to/your/project``` ==> specific folder, with your project dir specified.
