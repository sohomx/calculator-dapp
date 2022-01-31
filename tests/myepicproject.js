const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('mycalculatordapp', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.MyCalculatordapp;

  it('Creates a calculator', async() => {
    await program.rpc.create("Welcome to solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator]
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "Welcome to Solana");
    _calculator = calculator;
  })

  it('Creates a calculator', async() => {

  });

  it("Adds two numbers", async function() {

  });

  it("Multiplies two numbers", async function(){

  });

  it("Subtracts two numbers", async function() {

  });

  it("Divides two numbers", async function() {

  });
})