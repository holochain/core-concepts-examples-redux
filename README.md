# core-concepts-examples
A series of sample hApps (Holochain Apps) that explore core concepts for new Holochain developers

## Core Concepts:
1. Zome Functions: [HelloHolo](https://github.com/holochain/core-concepts-examples)
  - The simplest hApp possible... Use a Zome Function to return the string "Hello, Holo!"
1. Local Source Chain: [HelloMe](https://github.com/holochain/core-concepts-examples/tree/02-HelloMe)
  - An agent's private entries (i.e. data) are stored only on its source chain (and not replicated to the shared DHT)
1. Shared DHT: [HelloWorld](https://github.com/holochain/core-concepts-examples/tree/03-HelloWorld)
  - An agent's public entries are stored in its source chain as well as replicated to the shared DHT
1. Entry Validation: [HelloValid](https://github.com/holochain/core-concepts-examples/tree/04-HelloValid)
  - Entries are validated before they are stored (both in the source chain and in the DHT)
1. Links: [HelloBlog](https://github.com/holochain/core-concepts-examples/tree/05-HelloBlog)
  - Links can be used to provide other agents with awareness of the entries they want to find

