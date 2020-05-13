# ylq

ylq (pronounced "yoke") is a general purpose data format processor
inspired by jq.

or maybe

# dexser

(reverse of serde, references it, and is the order in which things
happen. First we deserialise then we transform (x), then we serialize

## The Juicy Details

Written in Rust and using the data serialization model
from Serde, we create a customized Serializer based on the input
command program. This Serializer is basically a big state-machine
that describes how to traverse and manipulate the data, and the whole
thing is mediated by the Serde interfaces
