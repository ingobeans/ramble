<img width="1920" height="419" alt="bannerthin" src="https://github.com/user-attachments/assets/67e26802-c543-4b27-8c36-84f79c17927b" />
Ramble is a little chaotic roguelike I made for the brackeys 2025 game jam. 

You're forced to deal with a chaos demon to get better gear, in turn making your run more and more chaotic as you rack up more chaos curses. These chaos curses can either be helpful to the player, or detrimental, so in a way, you have to risk it for the biscuit ;)

(the biscuit here being some cool piece of gear you want)

<img width="324" height="171" alt="Screenshot 2025-08-29 220234" src="https://github.com/user-attachments/assets/82987fc9-cf7e-4eb9-933b-393079eac158" />


## Controls

    [F] open inventory
    [Space] roll
    [Left Mouse Button] attack
    [WASD] move

The game is really inspired by tiny rogues as is probably apparent, as well as a bit of path of acra. The game also has no saving because its a roguelike (definitely not that i ran out of time and i otherwise wouldve implemented that)


## Building from source

this project is made in rust so obviously you'll need rust (with cargo) installed.

to run standalone you can just do:
```bash
cargo run
```

and to build for web and host on localhost with `basic-http-server`, do 
```bash
cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/ramble.wasm web/ && basic-http-server web/
```