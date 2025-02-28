import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TippingContract } from "../target/types/tipping_contract";
import { assert } from "chai";

//  B2r8THToCDRa3yevxgeZHAH6ppamdz7Y9sUfNSu2TVb8
describe("tipping_contract", () => {
  // Setup provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TippingContract as Program<TippingContract>;

  // Define keypair accounts
  const tipper = anchor.web3.Keypair.generate();
  const recipient = anchor.web3.Keypair.generate();
  const platformFeeAccount = anchor.web3.Keypair.generate();

  // Define fee percentage (e.g., 5% or 500 basis points)
  const FEE_BPS = new anchor.BN(500); // 5% fee
  const TIP_AMOUNT = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 10); // 0.1 SOL

  before(async () => {
    // Airdrop SOL to the tipper to cover tipping costs
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        tipper.publicKey,
        anchor.web3.LAMPORTS_PER_SOL // 1 SOL
      ),
      "confirmed"
    );
  });

  it("Tipping works correctly", async () => {
    // Get initial balances
    const tipperBalanceBefore = await provider.connection.getBalance(
      tipper.publicKey
    );
    const recipientBalanceBefore = await provider.connection.getBalance(
      recipient.publicKey
    );
    const platformBalanceBefore = await provider.connection.getBalance(
      platformFeeAccount.publicKey
    );

    // Send the tip
    await program.methods
      .tip(TIP_AMOUNT, FEE_BPS)
      .accounts({
        tipper: tipper.publicKey,
        recipient: recipient.publicKey,
        platformFeeAccount: platformFeeAccount.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
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
    const platformBalanceAfter = await provider.connection.getBalance(
      platformFeeAccount.publicKey
    );

    // Calculate expected transfers
    const feeAmount = (TIP_AMOUNT.toNumber() * FEE_BPS.toNumber()) / 10_000;
    const recipientAmount = TIP_AMOUNT.toNumber() - feeAmount;

    // Assertions
    assert(
      recipientBalanceAfter === recipientBalanceBefore + recipientAmount,
      "Recipient did not receive correct amount"
    );
    assert(
      platformBalanceAfter === platformBalanceBefore + feeAmount,
      "Platform did not receive correct fee"
    );
    assert(
      tipperBalanceAfter < tipperBalanceBefore - TIP_AMOUNT.toNumber(),
      "Tipper balance should decrease correctly"
    );
  });
});
