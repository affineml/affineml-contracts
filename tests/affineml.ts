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
import { base64 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

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

  const _buffer = crypto.createHash("sha256");
  _buffer.update(revealMsg);
  _buffer.update(signer.toBuffer());
  const __buffer = _buffer.digest();

  console.log(base64.encode(__buffer));
  console.log(Array.from(__buffer));

  const signers = [signer1, signer2, signer3, signer4, signer5];

  const program = anchor.workspace.Affineml as Program<Affineml>;

  const [confPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.Buffer.from("config", "utf8")],
    program.programId
  );

  const [mintPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.Buffer.from("mint", "utf8")],
    program.programId
  );

  it("airdrop!", async () => {
    await provider.connection.requestAirdrop(
      signer1.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1
    );
    await provider.connection.requestAirdrop(
      signer2.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1
    );
    await provider.connection.requestAirdrop(
      signer3.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1
    );
    await provider.connection.requestAirdrop(
      signer4.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1
    );
    await provider.connection.requestAirdrop(
      signer5.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 1
    );
  });

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

  const create = async (_taskId: number, round: number) => {
    const taskId = new anchor.BN(_taskId);

    const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("task", "utf8"),
        taskId.toArrayLike(Buffer.Buffer, "be", 8),
      ],
      program.programId
    );

    const tx = await program.methods
      .create(round)
      .accounts({
        config: confPDA,
        taskInfo: taskInfoPDA,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  };

  const commit = async (_taskId: number) => {
    const taskId = new anchor.BN(_taskId);

    const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.Buffer.from("task", "utf8"),
        taskId.toArrayLike(Buffer.Buffer, "be", 8),
      ],
      program.programId
    );

    for (let i = 0; i < signers.length; i++) {
      const signer = signers[i];

      const [userInfoPDA] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.Buffer.from("user", "utf8"),
          taskId.toArrayLike(Buffer.Buffer, "be", 8),
          signer.publicKey.toBuffer(),
        ],
        program.programId
      );

      const _buffer = crypto.createHash("sha256");
      _buffer.update(revealMsg);
      _buffer.update(signer.publicKey.toBuffer());
      const __buffer = _buffer.digest();

      await program.methods
        .commit({
          commitHash: Array.from(__buffer),
          taskIndex: taskId,
        })
        .accounts({
          taskInfo: taskInfoPDA,
          signer: signer.publicKey,
          userInfo: userInfoPDA,
        })
        .signers([signer])
        .rpc();
    }
  };

  const reveal = async (_taskId: number) => {
    try {
      const taskId = new anchor.BN(_taskId);

      const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.Buffer.from("task", "utf8"),
          taskId.toArrayLike(Buffer.Buffer, "be", 8),
        ],
        program.programId
      );

      const remains = [];
      for (let i = 0; i < signers.length; i++) {
        const signer = signers[i];

        const [userInfoPDA] = web3.PublicKey.findProgramAddressSync(
          [
            Buffer.Buffer.from("user", "utf8"),
            taskId.toArrayLike(Buffer.Buffer, "be", 8),
            signer.publicKey.toBuffer(),
          ],
          program.programId
        );
        remains.push({
          pubkey: userInfoPDA,
          isSigner: false,
          isWritable: true,
        });
      }

      await program.methods
        .reveal({
          taskIndex: taskId,
          revealInfo: Array.from(revealMsg),
        })
        .accounts({
          config: confPDA,
          taskInfo: taskInfoPDA,
        })
        .remainingAccounts(remains)
        .rpc();
      // const info = await program.account.taskInfo.fetch(taskInfoPDA);
      // console.log(info);
    } catch (error) {
      console.log(error);
    }
  };

  const claim = async (_taskId: number) => {
    try {
      const taskId = new anchor.BN(_taskId);

      const [taskInfoPDA] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.Buffer.from("task", "utf8"),
          taskId.toArrayLike(Buffer.Buffer, "be", 8),
        ],
        program.programId
      );

      for (let i = 0; i < signers.length; i++) {
        const signer = signers[i];
        const [userInfoPDA] = web3.PublicKey.findProgramAddressSync(
          [
            Buffer.Buffer.from("user", "utf8"),
            taskId.toArrayLike(Buffer.Buffer, "be", 8),
            signer.publicKey.toBuffer(),
          ],
          program.programId
        );

        const tokenAccount = getAssociatedTokenAddressSync(
          mintPDA,
          signer.publicKey,
          true
        );
        await program.methods
          .claim({
            taskIndex: taskId,
          })
          .accounts({
            config: confPDA,
            taskInfo: taskInfoPDA,
            mint: mintPDA,
            userTokenAccount: tokenAccount,
            signer: signer.publicKey,
            userInfo: userInfoPDA,
          })
          .signers([signer])
          .rpc();
      }
      const info = await program.account.taskInfo.fetch(taskInfoPDA);
      console.log(info);
    } catch (error) {
      console.log(error);
    }
  };

  it("create", async () => {
    await create(0, 1);
  });

  it("commit", async () => {
    await commit(0);
  });

  it("reveal", async () => {
    await reveal(0);
  });

  it("claim token reward", async () => {
    await claim(0);
  });

  it("multi round", async () => {
    await create(1, 5);
    for (let i = 0; i < 5; i++) {
      await commit(1);
      await reveal(1);
    }
    await claim(1);
  });
});
