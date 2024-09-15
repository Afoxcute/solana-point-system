import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { solana-point-system } from "../target/types/solana-point-system";
import { Clock, startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

describe("Quest refill", () => {
  test("Turning forward time will init quest agin", async () => {
    const context = await startAnchor(".", [], []);
    const client = context.banksClient;

    const provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    const program = anchor.workspace.solana-point-system as Program<solana-point-system>;
    const payer = provider.wallet as anchor.Wallet;
    const appDataSeed = "appData";

    const [userPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), payer.publicKey.toBuffer()],
      program.programId
    );

    const [appDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(appDataSeed)],
      program.programId
    );

    try {
      let tx = await program.methods
        .initUser(appDataSeed)
        .accountsStrict({
          user: userPDA,
          signer: payer.publicKey,
          appData: appDataPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc({ skipPreflight: true });
      console.log("Init transaction", tx);

      console.log("Confirmed", tx);
    } catch (e) {
      console.log("User already exists: ", e);
    }

   
    for (let i = 0; i < 11; i++) {
      console.log(`Taskinit instruction ${i}`);

      let tx = await program.methods
        .taskinit(appDataSeed, i)
        .accountsStrict({
          user: userPDA,
          sessionToken: null,
          signer: payer.publicKey,
          appData: appDataPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await console.log("Taskinit instruction", tx);
    }

    await client.getAccount(userPDA).then((info) => {
      const decoded = program.coder.accounts.decode(
        "userData",
        Buffer.from(info.data)
      );
      console.log("User account info", JSON.stringify(decoded));
      expect(decoded).toBeDefined();
      expect(parseInt(decoded.energy)).toEqual(89);
    });

    const timestamp = Math.floor(Date.now() / 1000);

    // Turn forward the clock for 11 minutes
    const currentClock = await client.getClock();
    context.setClock(
      new Clock(
        currentClock.slot,
        currentClock.epochStartTimestamp,
        currentClock.epoch,
        currentClock.leaderScheduleEpoch,
        BigInt(timestamp) + BigInt(60 * 11)
      )
    );


    let tx = await program.methods
      .taskInit(appDataSeed, 0)
      .accountsStrict({
        user: userPDA,
        sessionToken: null,
        signer: payer.publicKey,
        appData: appDataPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();


    await client.getAccount(userPDA).then((info) => {
      const decoded = program.coder.accounts.decode(
        "userData",
        Buffer.from(info.data)
      );
      console.log("User account info", JSON.stringify(decoded));
      expect(decoded).toBeDefined();
      expect(parseInt(decoded.energy)).toEqual(99);
    });
  });
});
