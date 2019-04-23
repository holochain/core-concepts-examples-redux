// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/core-concepts-examples.dna.json"
const dna = Config.dna(dnaPath)

const agentAlice = Config.agent("alice")
const instanceAlice = Config.instance(agentAlice, dna)

const scenario = new Scenario([instanceAlice])

scenario.runTape("description of example test", async (t, { alice }) => {

  // Make a call to a Zome function
  // Call get_message within message_zome, pass no parameters
  const result = await alice.callSync("message_zome", "get_message", {})
  console.log("Result:", result)

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: "Hello, Holo!" })
})
