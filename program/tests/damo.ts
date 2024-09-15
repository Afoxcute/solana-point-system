import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { solana-point-system } from "../target/types/solana-point-system";

describe("solana-point-system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.solana-point-system as Program<solana-point-system>;
  const payer = provider.wallet as anchor.Wallet;
  const appDataSeed = "appData";

  it("Init user and init point!", async () => {
    console.log("Local address", payer.publicKey.toBase58());

    const balance = await anchor
      .getProvider()
      .connection.getBalance(payer.publicKey);

    if (balance < 1e8) {
      const res = await anchor
        .getProvider()
        .connection.requestAirdrop(payer.publicKey, 1e9);
      await anchor
        .getProvider()
        .connection.confirmTransaction(res, "confirmed");
    }

    const [userPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), payer.publicKey.toBuffer()],
      program.programId
    );

    console.log("User PDA", userPDA.toBase58());

    const [userDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(userDataSeed)],
      program.programId
    );

    try {
      let tx = await program.methods
        .initUser(userDataSeed)
        .accountsStrict({
          user: userPDA,
          signer: payer.publicKey,
          userData: userDataPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc({ skipPreflight: true });
      console.log("Init transaction", tx);

      await anchor.getProvider().connection.confirmTransaction(tx, "confirmed");
      console.log("Confirmed", tx);
    } catch (e) {
      console.log("User already exists: ", e);
    }

    for (let i = 0; i < 11; i++) {
      console.log(`Init instruction ${i}`);

      let tx = await program.methods
        .taskInit(gameDataSeed, 0)
        .accountsStrict({
          user: userPDA,
          sessionToken: null,
          signer: payer.publicKey,
          userData: userDataPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log("Init instruction", tx);
      await anchor.getProvider().connection.confirmTransaction(tx, "confirmed");
    }

    const accountInfo = await anchor
      .getProvider()
      .connection.getAccountInfo(userPDA, "confirmed");

    const decoded = program.coder.accounts.decode(
      "userData",
      accountInfo.data
    );
    console.log("User account info", JSON.stringify(decoded));
  });
});
