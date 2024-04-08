import * as anchor from "@coral-xyz/anchor";
import {
  Program,
  web3,
  Wallet,
  AnchorProvider,
  BN,
  utils,
} from "@coral-xyz/anchor";
import { Affineml } from "../target/types/affineml";
import * as crypto from "crypto";
import {
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import * as Buffer from "buffer";
import {
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { util } from "chai";

describe("affineml", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const signer = provider.publicKey!;
  const signer1 = Keypair.generate();
  const signer2 = Keypair.generate();
  const signer3 = Keypair.generate();
  const signer4 = Keypair.generate();
  const signer5 = Keypair.generate();

  const revealMsg = new anchor.BN("11111").toArrayLike(Buffer.Buffer, "be", 32);

  const signers = [signer1, signer2, signer3, signer4, signer5];

  const program = anchor.workspace.Affineml as Program<Affineml>;

  console.log(program.programId.toBase58());

  const [confPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.Buffer.from("config", "utf8")],
    program.programId
  );

  const [mintPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.Buffer.from("mint", "utf8")],
    program.programId
  );

  const tokenAccount = getAssociatedTokenAddressSync(
    mintPDA,
    signer1.publicKey,
    true
  );


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .init()
      .accounts({
        mint: mintPDA,
        config: confPDA,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("create", async () => {
  //   const taskId = new anchor.BN("0");

  //   const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.Buffer.from("task", "utf8"),
  //       taskId.toArrayLike(Buffer.Buffer, "be", 8),
  //     ],
  //     program.programId
  //   );

  //   const tx = await program.methods
  //     .create()
  //     .accounts({
  //       config: confPDA,
  //       taskInfo: taskInfoPDA,
  //     })
  //     .rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("commit", async () => {
  //   const taskId = new anchor.BN("0");

  //   const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.Buffer.from("task", "utf8"),
  //       taskId.toArrayLike(Buffer.Buffer, "be", 8),
  //     ],
  //     program.programId
  //   );

  //   const buffer = crypto.createHash("sha256");

  //   for (let i = 0; i < signers.length; i++) {
  //     const signer = signers[i];

  //     const _buffer = crypto.createHash("sha256");
  //     _buffer.update(revealMsg);
  //     _buffer.update(signer.publicKey.toBuffer());
  //     const __buffer = _buffer.digest();

  //     await program.methods
  //       .commit({
  //         commitHash: Array.from(__buffer),
  //         taskIndex: taskId,
  //       })
  //       .accounts({
  //         taskInfo: taskInfoPDA,
  //         signer: signer.publicKey,
  //       })
  //       .signers([signer])
  //       .rpc();
  //   }
  // });

  // it("reveal", async () => {
  //   try {
  //     const taskId = new anchor.BN("0");

  //     const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.Buffer.from("task", "utf8"),
  //         taskId.toArrayLike(Buffer.Buffer, "be", 8),
  //       ],
  //       program.programId
  //     );

  //     console.log(Array.from(revealMsg))
  //     await program.methods
  //       .reveal({
  //         taskIndex: taskId,
  //         revealInfo: Array.from(revealMsg),
  //       })
  //       .accounts({
  //         config: confPDA,
  //         taskInfo: taskInfoPDA,
  //       })
  //       .rpc();

  //     const info = await program.account.taskInfo.fetch(taskInfoPDA);
  //     console.log(info);
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });

  // it("claim token reward", async () => {
  //   try {
  //     const taskId = new anchor.BN("0");

  //     const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.Buffer.from("task", "utf8"),
  //         taskId.toArrayLike(Buffer.Buffer, "be", 8),
  //       ],
  //       program.programId
  //     );

  //     const [claimPDA] = web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.Buffer.from("claim", "utf8"),
  //         new anchor.BN("0").toArrayLike(Buffer.Buffer, "be", 8),
  //         signer1.publicKey.toBuffer(),
  //       ],
  //       program.programId
  //     );
  //     console.log(claimPDA.toBase58());

  //     await program.methods
  //       .claim({
  //         taskIndex: taskId,
  //       })
  //       .accounts({
  //         config: confPDA,
  //         taskInfo: taskInfoPDA,
  //         mint: mintPDA,
  //         userTokenAccount: tokenAccount,
  //         signer: signer1.publicKey,
  //         claimInfo: claimPDA,
  //       })
  //       .signers([signer1])
  //       .rpc();

  //     const info = await program.account.taskInfo.fetch(taskInfoPDA);
  //     console.log(info);
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });
});
