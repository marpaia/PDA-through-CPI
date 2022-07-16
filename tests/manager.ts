import * as anchor from "@project-serum/anchor";

import { Manager } from "../target/types/manager";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { Worker } from "../target/types/worker";
import { expect } from "chai";

describe("manager", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const worker = anchor.workspace.Worker as Program<Worker>;
  const manager = anchor.workspace.Manager as Program<Manager>;

  it("Passes PDA through CPI!", async () => {
    const [dataPDA, _] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("worker"),
        provider.wallet.publicKey.toBuffer(),
      ],
      worker.programId
    );

    await worker.methods
      .initialize()
      .accounts({
        data: dataPDA,
        user: provider.wallet.publicKey,
      })
      .rpc();

    await worker.methods
      .setData(new anchor.BN(1337))
      .accounts({
        data: dataPDA,
      })
      .rpc();

    expect(
      (await worker.account.data.fetch(dataPDA)).value.toNumber()
    ).to.equal(1337);

    await manager.methods
      .setWorkerDataThroughCpi(new anchor.BN(42))
      .accounts({
        workerProgram: worker.programId,
        data: dataPDA,
      })
      .rpc();

    expect(
      (await worker.account.data.fetch(dataPDA)).value.toNumber()
    ).to.equal(42);
  });
});
