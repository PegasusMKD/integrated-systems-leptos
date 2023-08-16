# UI for the Integrated Systems subject using Leptos

For the "Integrated Systems" university project I have to create a FE using a separate project (aka don't use the built-in .NET templating).
With this project I intend to solve that, as well as:
 - Learn a bit of Rust
 - Learn to use Leptos
 - Learn how to deploy a Rust project

# Starting the project
If we assume you have everything installed, and you are on windows then you can run the ```start.ps1``` script, otherwise you can run the following commands:
```shell
npx tailwindcss -i ./input.css -o ./style/output.css --watch
cargo leptos watch
```
as separate commands.

In case you don't have something installed, please follow the guide in [this repo]:https://github.com/ThePrimeagen/orgwasm since the original configuration was taken from there and then updated/expanded on.

# Plans

This should be a sufficient list for required changes

 - [X] Implement base template (header/footer/base body)
 - [ ] Implement the Home Page
 - [ ] Implement the Index pages for each menu item in the navigation bar
 - [ ] Implement all required functionalities (CRUD essentially)
 - [ ] Test for bugs
