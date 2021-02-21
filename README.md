# serverless
simple serverless udp chat application

I wanted to learn about multithreading and networking so I chose to make a very simple chat application.
Right now, only the absolute minimal features are implemented. It works by sending the message as an UDP packet to the broadcast address, while simultaneously listening on 0.0.0.0 for incoming messages.
There are many things that could be added. I wanted to use UDP as it is simpler than TCP and does not requiere establishing a connection first.

### update 21.02.21

Added a basic xor cipher as an encryption mechanism. However, this is NOT secure and is not intended to be used for transmitting ANY sensible data as the encryption can be broken relatively easy.

On startup, the key is 0 so the sent text will be not encrypted. To set a key, simply type `key yourkey` in the prompt and replace `yourkey` with a key of your choice.
