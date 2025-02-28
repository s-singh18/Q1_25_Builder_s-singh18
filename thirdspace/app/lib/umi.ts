import * as fs from "fs";
import * as path from "path";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  Umi,
} from "@metaplex-foundation/umi";

export const getUmiFromFile = (): Umi => {
  const umi: Umi = createUmi("https://api.devnet.solana.com");

  // Use fs to navigate the filesystem till you reach
  // the wallet you wish to use via relative pathing.
  const walletFile = fs.readFileSync(path.join(__dirname, "./keypair.json"));

  // Usually Keypairs are saved as Uint8Array, so you
  // need to transform it into a usable keypair.
  const keypair = umi.eddsa.createKeypairFromSecretKey(
    new Uint8Array(walletFile)
  );

  // Before Umi can use this Keypair you need to generate
  // a Signer type with it.
  const signer = createSignerFromKeypair(umi, keypair);

  // Tell Umi to use the new signer.
  umi.use(signerIdentity(signer));

  console.log("UMI obj:", umi);

  return umi;
};
