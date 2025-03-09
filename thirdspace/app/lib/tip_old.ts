import * as anchor from "@project-serum/anchor";
import { readFileSync } from "fs";

console.log("Running tip contract!");

const wallet = new anchor.Wallet(
  anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(
      JSON.parse(
        readFileSync(
          "/Users/sss/Turbin3/Q1_25_Builder_s-singh18/thirdspace/Turbin3-wallet.json",
          "utf-8"
        )
      )
    )
  )
);
const provider = new anchor.AnchorProvider(
  new anchor.web3.Connection("https://api.devnet.solana.com"),
  wallet,
  {}
);
anchor.setProvider(provider);

const idl = JSON.parse(
  readFileSync(
    "/Users/sss/Turbin3/Q1_25_Builder_s-singh18/thirdspace/target/idl/vault.json",
    "utf8"
  )
);
console.log("✅ IDL Loaded:", JSON.stringify(idl, null, 2));

const programID: string = "2GdsDdH2jbDMVs7k5jPKZur2hEMaPm5sJA2DHMtRs8zN";
const program = new anchor.Program(idl, programID);
console.log("✅ Successfully loaded program:", programID.toString());

export const tip = async () => {
  try {
    const tipper = new anchor.web3.PublicKey(
      "B2r8THToCDRa3yevxgeZHAH6ppamdz7Y9sUfNSu2TVb8"
    );
    const recipient = new anchor.web3.PublicKey(
      "BBkGcjr14MDp5qQQgEfsJ9bXPMW2Qy5Fpd8Gpc8HbAAm"
    );
    const vault = new anchor.web3.PublicKey(
      "2GdsDdH2jbDMVs7k5jPKZur2hEMaPm5sJA2DHMtRs8zN"
    );
    const [vaultStatePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), tipper.toBuffer()],
      new anchor.web3.PublicKey(programID)
    );

    const result = await program.methods
      .tip(new anchor.BN(1000000), new anchor.BN(500)) // 1 SOL, 5% fee
      .accounts({
        tipper: tipper,
        recipient: recipient,
        vault: vault,
        vaultState: new anchor.web3.PublicKey(vaultStatePDA),
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    return result;
  } catch (error) {
    console.error("Error tipping:", error);
  }
};
