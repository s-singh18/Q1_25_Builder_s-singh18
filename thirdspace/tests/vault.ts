import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { assert } from "chai";
import { readFileSync } from "fs";

// Helper function to load a keypair from a file
function loadKeypair(filepath: string): anchor.web3.Keypair {
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(readFileSync(filepath, "utf-8")))
  );
}

describe("vault tipping system", () => {
  // Set up the provider and program
  const provider = anchor.AnchorProvider.local("https://api.devnet.solana.com");
  anchor.setProvider(provider);
  const program = anchor.workspace.Vault as Program<Vault>;

  // Generate keypairs for users
  const tipper = loadKeypair("/Users/sss/.config/solana/id.json");
  const recipient = loadKeypair(
    "/Users/sss/Turbin3/Q1_25_Builder_s-singh18/thirdspace/Turbin3-wallet.json"
  );
  const admin = loadKeypair("/Users/sss/.config/solana/id.json");

  // Derive PDAs for vault and vault state
  let vaultStatePda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let vaultBump: number;

  // Define transaction parameters
  const FEE_BPS = new anchor.BN(500); // 5% fee
  const TIP_AMOUNT = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 10); // 0.1 SOL
  const SYSTEM_PROGRAM = anchor.web3.SystemProgram.programId;

  before(async () => {
    // Airdrop SOL to the tipper and admin to fund transactions
    //   await provider.connection.confirmTransaction(
    //     await provider.connection.requestAirdrop(
    //       tipper.publicKey,
    //       anchor.web3.LAMPORTS_PER_SOL // 1 SOL
    //     ),
    //     "confirmed"
    //   );

    //   await provider.connection.confirmTransaction(
    //     await provider.connection.requestAirdrop(
    //       admin.publicKey,
    //       anchor.web3.LAMPORTS_PER_SOL // 1 SOL
    //     ),
    //     "confirmed"
    //   );

    // Find PDAs
    [vaultStatePda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), tipper.publicKey.toBuffer()],
      program.programId
    );
    [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultStatePda.toBuffer()],
      program.programId
    );

    // Initialize the vault
    try {
      await program.methods
        .initialize()
        .accounts({
          user: tipper.publicKey,
          state: vaultStatePda,
          vault: vaultPda,
          systemProgram: SYSTEM_PROGRAM,
        })
        .signers([tipper])
        .rpc();
    } catch (error) {
      console.error("Initialization error:", error);
    }
  });

  it("Tipper successfully tips the recipient with a percentage fee going to the vault", async () => {
    // Get initial balances
    const tipperBalanceBefore = await provider.connection.getBalance(
      tipper.publicKey
    );
    const recipientBalanceBefore = await provider.connection.getBalance(
      recipient.publicKey
    );
    const vaultBalanceBefore = await provider.connection.getBalance(vaultPda);

    // Send tip
    await program.methods
      .tip(TIP_AMOUNT, FEE_BPS)
      .accounts({
        tipper: tipper.publicKey,
        recipient: recipient.publicKey,
        vault: vaultPda,
        vaultState: vaultStatePda,
        systemProgram: SYSTEM_PROGRAM,
      })
      .signers([tipper])
      .rpc();

    // Get new balances
    const tipperBalanceAfter = await provider.connection.getBalance(
      tipper.publicKey
    );
    const recipientBalanceAfter = await provider.connection.getBalance(
      recipient.publicKey
    );
    const vaultBalanceAfter = await provider.connection.getBalance(vaultPda);

    // Calculate expected values
    const feeAmount = (TIP_AMOUNT.toNumber() * FEE_BPS.toNumber()) / 10_000;
    const recipientAmount = TIP_AMOUNT.toNumber() - feeAmount;

    // Assertions
    assert(
      recipientBalanceAfter === recipientBalanceBefore + recipientAmount,
      "Recipient did not receive the correct amount"
    );
    assert(
      vaultBalanceAfter === vaultBalanceBefore + feeAmount,
      "Vault did not receive the correct platform fee"
    );
    assert(
      tipperBalanceAfter < tipperBalanceBefore - TIP_AMOUNT.toNumber(),
      "Tipper's balance should decrease by at least the tipped amount"
    );
  });

  //   it("Admin successfully withdraws accumulated fees from the vault", async () => {
  //     // Get vault balance before withdrawal
  //     const vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
  //     const adminBalanceBefore = await provider.connection.getBalance(
  //       admin.publicKey
  //     );

  //     // Withdraw fees
  //     await program.methods
  //       .withdrawFees(new anchor.BN(vaultBalanceBefore))
  //       .accounts({
  //         admin: admin.publicKey,
  //         vault: vaultPda,
  //         vaultState: vaultStatePda,
  //         systemProgram: SYSTEM_PROGRAM,
  //       })
  //       .signers([admin])
  //       .rpc();

  //     // Get balances after withdrawal
  //     const vaultBalanceAfter = await provider.connection.getBalance(vaultPda);
  //     const adminBalanceAfter = await provider.connection.getBalance(
  //       admin.publicKey
  //     );

  //     // Assertions
  //     assert(
  //       vaultBalanceAfter === 0,
  //       "Vault should be empty after withdrawing all fees"
  //     );
  //     assert(
  //       adminBalanceAfter > adminBalanceBefore,
  //       "Admin did not receive withdrawn fees"
  //     );
  //   });
});
