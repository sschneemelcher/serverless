# serverless
simple serverless udp chat application

I wanted to learn about multithreading and networking so I chose to make a very simple chat application.
Right now, only the absolute minimal features are implemented. It works by sending the message as an UDP packet to the broadcast address, while simultaneously listening on 0.0.0.0 for incoming messages.
There are many things that could be added. I wanted to use UDP as it is simpler than TCP and does not requiere establishing a connection first.
However, it uses no encryption, so this is not supposed to be used for any purpose that involves ANY kind of sensible data as your messages are broadcasted in plain text in your local network.
