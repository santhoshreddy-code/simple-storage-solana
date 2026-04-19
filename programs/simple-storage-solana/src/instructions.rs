import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleStorageSolana } from "../target/types/simple_storage_solana";
import { expect } from "chai";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("simple-storage-solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .simpleStorageSolana as Program<SimpleStorageSolana>;

  const storageAccount = Keypair.generate();

  it("should initialize storage with owner and zero", async () => {
    await program.methods
      .initialize()
      .accounts({
        storage: storageAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([storageAccount])
      .rpc();

    const account = await program.account.storageData.fetch(
      storageAccount.publicKey
    );

    expect(account.owner.toString()).to.equal(
      provider.wallet.publicKey.toString()
    );
    expect(account.storedNumber.toNumber()).to.equal(0);
  });

  it("should allow owner to set a new number", async () => {
    await program.methods
      .setNumber(new anchor.BN(42))
      .accounts({
        storage: storageAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.storageData.fetch(
      storageAccount.publicKey
    );

    expect(account.storedNumber.toNumber()).to.equal(42);
  });

  it("should reject non-owner from setting a number", async () => {
    const fakeUser = Keypair.generate();

    const airdropSig = await provider.connection.requestAirdrop(
      fakeUser.publicKey,
      1_000_000_000
    );
    await provider.connection.confirmTransaction(airdropSig);

    try {
      await program.methods
        .setNumber(new anchor.BN(999))
        .accounts({
          storage: storageAccount.publicKey,
          user: fakeUser.publicKey,
        })
        .signers([fakeUser])
        .rpc();

      expect.fail("Should have thrown an error");
    } catch (error) {
      expect(error.toString()).to.include("NotOwner");
    }
  });

  it("should return the correct stored number", async () => {
    await program.methods
      .setNumber(new anchor.BN(777))
      .accounts({
        storage: storageAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.storageData.fetch(
      storageAccount.publicKey
    );

    expect(account.storedNumber.toNumber()).to.equal(777);
  });
});