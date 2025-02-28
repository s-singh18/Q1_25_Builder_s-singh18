import {
  none,
  Umi,
  createSignerFromKeypair,
  // generateSigner,
  // percentAmount,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import {
  getAssetWithProof,
  updateMetadata,
  UpdateArgsArgs,
} from "@metaplex-foundation/mpl-bubblegum";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import wallet from "../Turbin3-wallet.json";

const umi: Umi = createUmi("https://api.devnet.solana.com");

try {
  const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
  const signer = createSignerFromKeypair(umi, keypair);

  // Tell Umi to use the new signer.
  umi.use(signerIdentity(signer));
  // umi.use(mplToolbox());  // Used for create collection cNFTs
} catch (error) {
  console.error("❌ Error reading wallet file:", error.message);
}

export const updateCNFT = async (assetId: string, ownerId: string) => {
  const assetPubKey = publicKey(assetId);
  const leafOwner = publicKey(ownerId);

  // Fetch the asset and proof.
  const assetWithProof = await getAssetWithProof(umi, assetPubKey, {
    truncateCanopy: true,
  });

  // Use it to update metadata for the NFT.
  const updateArgs: UpdateArgsArgs = {
    name: "New name",
    uri: "https://updated-example.com/my-nft.json",
  };
  const result = await updateMetadata(umi, {
    ...assetWithProof,
    leafOwner,
    currentMetadata: assetWithProof.metadata,
    updateArgs,
    // Optional param. If your authority is a different signer type
    // than the current umi identity assign that signer here.
    // authority: <Signer>
    // Optional param. If cNFT belongs to a collection pass it here.
    // collectionMint: publicKey("22222222222222222222222222222222"),
  }).sendAndConfirm(umi);

  console.log("Result:", result);
};
