import {
  Umi,
  createSignerFromKeypair,
  // generateSigner,
  // percentAmount,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";

import { burnV1 } from "@metaplex-foundation/mpl-token-metadata";

const umi: Umi = createUmi("https://api.devnet.solana.com");

try {
  const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
  const signer = createSignerFromKeypair(umi, keypair);

  // Tell Umi to use the new signer.
  umi.use(signerIdentity(signer));
  umi.use(dasApi());
  // umi.use(mplToolbox());  // Used for create collection cNFTs
} catch (error) {
  const errorMessage = error instanceof Error ? error.message : String(error);
  console.error("❌ Error reading wallet file:", errorMessage);
}
