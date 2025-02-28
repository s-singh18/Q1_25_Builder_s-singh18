import {
  Umi,
  createSignerFromKeypair,
  // generateSigner,
  // percentAmount,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  findLeafAssetIdPda,
  fetchMerkleTree,
} from "@metaplex-foundation/mpl-bubblegum";
import wallet from "../Turbin3-wallet.json";

import { dasApi } from "@metaplex-foundation/digital-asset-standard-api";

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

export const getAsset = async (assetId: string) => {
  try {
    // const assetId = "AwuMq9MLumGCbqGAe91JFffXuqQP2G3j8C8oMtFEqkiW";
    const pubKey = publicKey(assetId);

    const asset = await umi.rpc.getAsset(pubKey);
    console.log(asset);
    return asset;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error reading wallet file:", errorMessage);
    throw new Error(errorMessage);
  }

  //   const rpcAsset = await umi.rpc.getAsset(assetId)
};

export const getAssetProof = async (assetId: string) => {
  try {
    // const assetId = "AwuMq9MLumGCbqGAe91JFffXuqQP2G3j8C8oMtFEqkiW";
    const pubKey = publicKey(assetId);

    const proof = await umi.rpc.getAssetProof(pubKey);
    console.log(proof);
    return proof;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error reading wallet file:", errorMessage);
    throw new Error(errorMessage);
  }
};

export const getAssetByOwner = async ({
  ownerId,
}: //   other params: sortBy, page, before, after
{
  ownerId: string;
}) => {
  try {
    const owner = publicKey(ownerId);

    const proof = await umi.rpc.getAssetsByOwner({
      owner,
    });
    console.log(proof);
    return proof;
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error reading wallet file:", errorMessage);
    throw new Error(errorMessage);
  }
};

// export const getAssetsByGroup = async ({}) => {}  // get asset by NFT collection
