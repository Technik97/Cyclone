# Project Cyclone
This is a project using Rust backend & Svelte Typescript frontend,the motive behind it is to create a template for basic feature(s) that can be used for applications that use Rust backend & Svelte frontend. 
I have started with implementing User Authentication, in which user will be able to sign-up and login, Sign up & login forms will be Implemented using Svelte Typescript frontend and backend api microservice will be implemented using Rust and Actix-Web with Postgres as database choice along with Redis for storing cookies that'll manage auth sessions.

----
### Tech Stack

- #### BackEnd Stack
	- Rust Programming Language.
	- Actix Web (Web Framework)
	- Sea Orm (ORM)
	- Other Rust crates like serde, tokio, chrono etc.

- #### FrontEnd Stack
	- Typescript programming Language
	- Svelte (Frontend Web Framework)
	- TailwindCSS (CSS Framework)
	- AlpineJS (optional, for JS behavior)
	- Other javascript libraries like axios

- #### Other Requirements
	- Postgres
	- Redis
	- Docker
	- Sqlx CLI

---

check readme files in respective folders for setting api & web.

---

Note :-  This project is heavily opinionated, I make no claims that this is the right way of doing things. The goal is to learn things by implementing.
