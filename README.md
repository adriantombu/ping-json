# A ping to JSON crate

This crate-to-be converts the output of the `ping` command to a json string.

This is just a random side project I created to start learning Rust and its ecosystem so it will probably end up as a crate in the end to teach me how to do it.

### How to use

To be determined

### Output on success

`{"host":"otso.fr","ip":"104.21.3.82","packets_send":3,"packets_received":3,"packet_loss":0.0,"min":60.785,"avg":61.684,"max":63.054,"stddev":0.985}`

### Output on error

`{"error":"ping: cannot resolve otso.404: Unknown host\n"}`
