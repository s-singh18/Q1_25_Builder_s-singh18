import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { BN } from "bn.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("anchor_escrow", () => {
  it("lets make an Escrow!", async () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;
    const maker = anchor.web3.Keypair.generate();
    const taker = anchor.web3.Keypair.generate();
    const mintA = anchor.web3.Keypair.generate();
    const mintB = anchor.web3.Keypair.generate();
    const seed = new BN(randomBytes(0));
    const tokenProgram = TOKEN_PROGRAM_ID;

    const makerAtaA = getAssociatedTokenAddressSync(
      maker.publicKey,
      mintA.publicKey,
      false,
      tokenProgram
    );

    const escrow = PublicKey.findProgramAddressSync([Buffer]);

    const accounts = {
      maker,
      mint_a,
      mint_b,
      maker_ata_a,
      escrow,
      vault,
      associated_token,
    };

    const tx = await program.methods
      .make(new BN(1), new BN(1), new BN(1))
      .accountsPartial({})
      .rpc();
  });
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
