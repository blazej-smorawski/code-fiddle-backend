# code-fiddle-client

- [code-fiddle-client](#code-fiddle-client)
  - [Architecture](#architecture)
  - [Development](#development)

I'm working on a small coding competition for primary school students and I wanted 
to create seamless environment in browser for them to learn without the need to
install IDEs or commandline tools.

## Architecture

The idea is to make the service both safe and easy to tune to various needs,
so I'd like to dispatch user code into docker containers and run it in a
safe and consistent environment. Also, I want to keep it as simple as possible.

## Development

The project is in very (very) early phase. These are milestones for version 0.0.1:

- [X] reading JSON input from front-end
- [ ] running user code in docker container
- [ ] code evaluation