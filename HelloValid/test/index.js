// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

// Create a reference to our DNA microservice
const dnaPath = "./dist/HelloValid.dna.json"
const dna = Config.dna(dnaPath)
// Create an agent, alice, within our DNA microservice
const agentAlice = Config.agent("alice")
const instanceAlice = Config.instance(agentAlice, dna)
// Define our scenario with alice and bob
const scenario = new Scenario([instanceAlice])

scenario.runTape("fail posting an invalid message", async (t, { alice }) => {

  // The BAD message - i.e. it does not contain the string "Hello"
  const badMessage = "Hi, Alice!";
  const expectedError = { Err: { Internal: '{"kind":{"ValidationFailed":"Messages must contain \\"Hello\\"."},"file":"/Users/travis/build/holochain/holochain-rust/core/src/nucleus/ribosome/runtime.rs","line":"192"}' } }

  // Alice tries to create an invalid message
  console.log("Bad Message:", badMessage)
  const addr = alice.call("message_zome", "create_message", {"message_string" : badMessage })
  console.log("Bad Message Address:", addr)

  // check that Alice gets the correct error
  t.deepEqual(addr, expectedError)
})
