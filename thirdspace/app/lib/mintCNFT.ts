import {
  none,
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
  mintV1,
  parseLeafFromMintV1Transaction,
  LeafSchema,
} from "@metaplex-foundation/mpl-bubblegum";
// import { createNft } from "@metaplex-foundation/mpl-token-metadata";
// import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";
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

export const mintCNFT = async (/*leafOwnerAddr, merkleTreeAddr*/) => {
  const leafOwnerAddr = "BBkGcjr14MDp5qQQgEfsJ9bXPMW2Qy5Fpd8Gpc8HbAAm";
  const merkleTreeAddr = "CK6tUG2ktsuRignn5cKYTSqTAS1rs977hAcLT45AjDYk";

  const leafOwner = publicKey(leafOwnerAddr);
  const merkleTree = publicKey(merkleTreeAddr);
  try {
    const { signature } = await mintV1(umi, {
      leafOwner,
      merkleTree,
      metadata: {
        name: "My Compressed NFT",
        uri: "https://example.com/my-cnft.json",
        sellerFeeBasisPoints: 500, // 5%
        collection: none(),
        creators: [
          { address: umi.identity.publicKey, verified: false, share: 100 },
        ],
      },
    }).sendAndConfirm(umi);

    const leaf: LeafSchema = await parseLeafFromMintV1Transaction(
      umi,
      signature
    );
    const assetId = findLeafAssetIdPda(umi, {
      merkleTree,
      leafIndex: leaf.nonce,
    });

    console.log("✅ Minted cNFT:", signature);
    console.log("✅ Asset ID:", assetId);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    console.error("❌ Error fetching Merkle tree:", errorMessage);
    throw new Error(errorMessage);
  }
};

/*
// Finish up NFT collection l8r

export const createCollectionNFT = async () => {
  const collectionMint = generateSigner(umi);
  const nft = await createNft(umi, {
    mint: collectionMint,
    name: "My Collection",
    uri: "https://example.com/my-collection.json",
    sellerFeeBasisPoints: percentAmount(5.5), // 5.5%
    isCollection: true,
  }).sendAndConfirm(umi);

  console.log(nft);

  return nft;
};

export const mintCNFTCollection = () => {
  const leafOwnerAddr = "BBkGcjr14MDp5qQQgEfsJ9bXPMW2Qy5Fpd8Gpc8HbAAm";
  const merkleTreeAddr = "CK6tUG2ktsuRignn5cKYTSqTAS1rs977hAcLT45AjDYk";

  const leafOwner = publicKey(leafOwnerAddr);
  const merkleTree = publicKey(merkleTreeAddr);

  await mintToCollectionV1(umi, {
    leafOwner,
    merkleTree,
    collectionMint,
    metadata: {
      name: "My Compressed NFT",
      uri: "https://example.com/my-cnft.json",
      sellerFeeBasisPoints: 500, // 5%
      collection: { key: collectionMint, verified: false },
      creators: [
        { address: umi.identity.publicKey, verified: false, share: 100 },
      ],
    },
  }).sendAndConfirm(umi);
};
*/
