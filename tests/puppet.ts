import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Puppet } from "../target/types/puppet";
import { PuppetMaster } from "../target/types/puppet_master";
import { expect } from "chai";
describe("puppet", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace
    .PuppetMaster as Program<PuppetMaster>;
  const puppetKeyPair = anchor.web3.Keypair.generate();
  const authorityKeyPair = anchor.web3.Keypair.generate();

  it("Dose CPI", async () => {
    await puppetProgram.methods
      .initialize(authorityKeyPair.publicKey)
      .accounts({
        puppet: puppetKeyPair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([puppetKeyPair])
      .rpc();

    const res = await puppetMasterProgram.methods
      .pullStrings(new anchor.BN(42))
      .accounts({
        puppet: puppetKeyPair.publicKey,
        authority: authorityKeyPair.publicKey,
      })
      .signers([authorityKeyPair])
      .rpc();

    expect(
      (
        await puppetProgram.account.data.fetch(puppetKeyPair.publicKey)
      ).data.toNumber()
    ).to.equal(42);
  });
});
