## Sprinkler Controller Suite

This is my foray into embedded software development on the ESP32. I'm reaching a bit here, but my end goal is to be running bare metal Rust in a `no-std` environment on the ESP32-C6. I'll be hosting the websocket server and the dashboard on my Raspberry Pi, and the controller will be running in my garage.

### Components

| Application  | Description                                                                                                                                                                                                                   |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `controller` | This is the code running on the ESP32. The goal is to connect to the `websocket-server` using `ws://` protocol which will then provide an interface for the user (via the dashboard) and the controller to talk to eachother. |
| `server`     | A lightweight server written using `tokio-tungstenite` to relay messages back and forth between the controller and the dashboard.                                                                                             |
| `dashboard`  | A basic Next.js application that will serve as a simple user interface for interacting with the controller, as well as displaying information streaming from the controller to the user.                                      |
| `shared`     | Rust type definitions for server-controller messages.                                                                                                                                                                         |

### Future Ideas

I'll just be using this for notes on what I may want to implement to go with this in the future.

- `wss://` connections - This would be nice if I plan on hosting this on the world wide web later. Will need to integrate some kind of authentication system as well to make sure that people aren't messing with my sprinkler configurations once I get them dialed in.
- Redis database & data collection - I'm not sure what "real time metrics" I could be displaying other than things like "sprinkler zone x activated", but might be a nice experiment to pipe everything through a Redis database just to have live updates on everything & the accompanying event history, instead of relying on a live websocket connection.
