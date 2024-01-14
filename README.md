# RATT!
*Ratt* is a network chat application built to be *fast* and *portable* in, you guessed it, rust.

# Installation
Ratt can be run using the server and ratt executables in the releases tab.

# Use
If you are the host of the chat 'room', or the server, then you must run the server executable. You will be prompted to enter the room's name and it's address using ip:port notation. You will also be asked for the max connections the room can handle, each connection requires one thread. The server does some minimal logging but everything important is also sent to the client.

Now, if you are a user, then you must run the ratt executable in order to join the room. You will similarly be prompted for your username and the room's address, again using ip:port notation. Once connected the screen will be cleared and you can start chatting!
> Running the ratt executable requires two threads, be careful running multiple users on one computer.

# Features
- Vibrant yet simple text based user interface
- Safe: Server will keep chugging regardless of what the client does
- Portable and can run on any platform

