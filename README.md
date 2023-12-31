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
 - [X] Implement the Index pages for each menu item in the navigation bar
 - [X] View Slot Index Page
 - [X] View Slot Create Page
 - [X] View Slot Edit Page
 - [X] Authentication
 - [X] View Slot Tickets Pages
 - [X] Tickets Index Page
 - [X] Tickets Export Page (with all related functionality)
 - [X] Baseline the tickets page
 - [X] Connect directly to BE
 - [X] Deserialize data since keys are using different cases
 - [X] Style the table
 - [X] Implement "Shopping Cart Index Page"
 - [X] Implement Stripe (https://docs.rs/stripe-rust/latest/stripe/)
 - [X] Implement Orders pages
 - [X] Implement Users Management page
 - [X] Implement all required functionalities (CRUD essentially)
 - [X] Implement different displays based on user role
 - [X] Test for bugs
 - [X] Implement register 
 - [X] Implement logout
 - [X] Implement Dockerfile for deployment

In regards to deployment:

 - [X] Find a good site to deploy Rust on (for now DigitalOcean looks like a decent choice, do more research on YT though) - works well with Digital Ocean Droplets
 - [X] Deploy BE on Azure if still have credits - deployed on Digital Ocean Kubernetes Cluster
 - [X] Deploy Rust manually first
 - [X] Add kubernetes files
 - [X] Add .dockerignore file
 - [X] Setup github actions for deployment
 - [X] Add 'push-to-deploy' code to the pipelines
 - [X] Test out deployment with actual change on application
 - [X] Mimic on BE side as well

Also in regards to maintanance:

 - [X] Clean-up all/most leptos & Rust warnings
 - [X] Remove hard-coded BE server path and add some extra configuration on top of it
 - [X] Add time-out for token (aka, automatically clean-up invalid sessions)
 - [X] Figure out problem with User Role Claims - token was just expired most likely, works fine now


# Bonus

Things that I'd like to add if I have time or feel like doing:
 - [ ] Make components more generic
 - [ ] Implement a # column to the tables which properly updates, but only itself gets updated, not entire DOM of the table
 - [ ] Re-organize the code-base to be easier to navigate and access structs/traits/impls
 - [ ] Add "redirects" if user is not authenticated - Partially implemented 
 - [ ] Implement user details page (page where the user can update their own information)
 - [ ] Implement logic for config which determines which deployment.toml it should use

 [this repo]: https://github.com/ThePrimeagen/orgwasm
 [this project]: https://github.com/PegasusMKD/Integrated-Systems-Homework
