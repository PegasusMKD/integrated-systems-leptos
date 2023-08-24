# UI for the Integrated Systems subject using Leptos

For the "Integrated Systems" university project I have to create a FE using a separate project (aka don't use the built-in .NET templating).
With this project I intend to solve that, as well as:
 - Learn a bit of Rust
 - Learn to use Leptos
 - Learn how to deploy a Rust project
 - Get acquinted with using NeoVim

# Technologies being used
A few of the technologies/libraries that are currently in the project
 - Leptos
 - TailwindCSS
 - serde
 - chrono
 - reqwest
 - WASM
 - Actix (mainly for serving the site and CSS since I am using SSR)

# End goal of the project
To serve as the FE UI for [this project] of course, with some changes in said project as well. I plan to have both of them running separately and then have this project send over requests when needed for a component. Would've been nice if I also re-wrote the BE logic into Rust using actix or axum but what you gonna do, them's the rules :(

# Starting the project
If we assume you have everything installed, and you are on windows then you can run the ```start.ps1``` script, otherwise you can run the following commands:
```shell
npx tailwindcss -i ./input.css -o ./style/output.css --watch
cargo leptos watch
```
as separate commands.

In case you don't have something installed, please follow the guide in [this repo] since the original configuration was taken from there and then updated/expanded on.

# Plans

This should be a sufficient list for required changes

 - [X] Implement base template (header/footer/base body)
 - [X] Implement the Home Page
 - [ ] Implement the Index pages for each menu item in the navigation bar - IN PROGRESS
 - [X] View Slot Index Page
 - [X] View Slot Create Page
 - [X] View Slot Edit Page
 - [ ] Authentication - IN PROGRESS
 - [ ] View Slot Tickets Pages
 - [X] Tickets Index Page
 - [X] Tickets Export Page (with all related functionality)
 - [X] Baseline the tickets page
 - [X] Connect directly to BE
 - [X] Deserialize data since keys are using different cases
 - [X] Style the table
 - [ ] Implement all required functionalities (CRUD essentially)
 - [ ] Test for bugs

# Bonus

Things that I'd like to add if I have time or feel like doing:
 - [ ] Make components more generic
 - [ ] Implement a # column to the tables which properly updates, but only itself gets updated, not entire DOM of the table


 [this repo]: https://github.com/ThePrimeagen/orgwasm
 [this project]: https://github.com/PegasusMKD/Integrated-Systems-Homework
